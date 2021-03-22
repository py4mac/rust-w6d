mod handlers;
mod models;
extern crate env_logger;
use actix_web::{App, HttpServer};
use crate::handlers::*;
use actix_web::middleware::Logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(train)
            .service(infer)
    })
    .bind("0.0.0.0:80")?
    .run()
    .await
}