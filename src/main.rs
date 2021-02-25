use std::net::TcpListener;

use zero2prod::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("[fe80::9d27:3ea:17de:2516%9]:8000")?;
    run(listener)?.await
}
