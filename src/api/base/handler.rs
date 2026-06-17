use actix_web::{HttpResponse, Responder};
use serde_json::json;

pub async fn app_info() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "name": env!("CARGO_PKG_NAME"),
        "description": env!("CARGO_PKG_DESCRIPTION"),
        "version": env!("CARGO_PKG_VERSION")
    }))
}

pub async fn health() -> impl Responder {
    HttpResponse::NotImplemented().json(json!({
        "Info" : "Not Implemented yet"
    }))
}
