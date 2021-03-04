use sqlx::postgres::PgPool;
use std::net::TcpListener;

use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::init_subscriber;
use zero2prod::telemetry::get_subscriber;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let subscriber = get_subscriber("zero2prod".into(), "info".into());
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_string = configuration.database.connection_string();
    let connection_pool = PgPool::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");

    let address = format!("[::1]:{}", configuration.application_port);

    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await?;

    Ok(())
}
