use sqlx::{PgPool};
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let configuration = zero2prod::configuration::get_configuration().expect("Failed to read configuration.");
    let connection_string = configuration.database.connection_string();
    let connection_pool = PgPool::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");

    let address = format!("[::1]:{}", configuration.application_port);

    let listener = TcpListener::bind(address)?;
    zero2prod::startup::run(listener, connection_pool)?.await
}
