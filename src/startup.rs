use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use tracing_actix_web::TracingLogger;
use sqlx::{PgPool};
use std::net::TcpListener;

use crate::routes::*;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger)
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .route("/{name}", web::get().to(greet))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
