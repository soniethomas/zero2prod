use std::net::TcpListener;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use zero2prod::configuration::DatabaseSettings;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

#[actix_rt::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
            .get(&format!("{}/health_check", &app.address))
            .send()
            .await
            .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[actix_rt::test]
async fn subscribe_returns_a_200_for_valid_form_data() {

    let app = spawn_app().await;

    let client = reqwest::Client::new();

    let body = "name=S%20Thomas&email=test.email%40example.com";

    let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "test.email@example.com");
    assert_eq!(saved.name, "S Thomas");
}

#[actix_rt::test]
async fn subscribe_returns_a_400_when_data_is_missing() {

    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=S%20Thomas", "missing email"),
        ("email=test.email%40example.com", "missing name"),
        ("", "missing both email and name")
    ];

    for(invalid_body, error_message) in test_cases {
        let response = client
                .post(&format!("{}/subscriptions", &app.address))
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(invalid_body)
                .send()
                .await
                .expect("Failed to execute request.");

        assert_eq!(
            400, response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.", error_message
        );
    }
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("[::1]:0").expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();

    let mut configuration = zero2prod::configuration::get_configuration().expect("Failed to read configuration");
    configuration.database.database_name = Uuid::new_v4().to_string();
    
    // let connection_string = configuration.database.connection_string();
    let connection_pool = configure_database(&configuration.database).await;

    let server = zero2prod::startup::run(listener, connection_pool.clone()).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    let address = format!("http://[::1]:{}", port);

    TestApp {
        address,
        db_pool: connection_pool
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name))
        .await
        .expect("Failed to create database.");

    println!("DB {}", &config);

    // Migrate database
    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}
