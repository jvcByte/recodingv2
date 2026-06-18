mod api;
mod shared;

use actix_web::{
    App, HttpServer,
    middleware::{Logger, NormalizePath},
};
use std::io::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    shared::config::logger::setup_logger();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(shared::config::cors::cors())
            .wrap(NormalizePath::trim())
            .configure(api::routes)
    })
    .workers(1)
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
