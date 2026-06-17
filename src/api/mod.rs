pub mod base;
use crate::api::base::routes::base_routes;
use actix_web::{HttpResponse, Responder, web};

pub async fn unmatched() -> impl Responder {
    HttpResponse::NotFound().json(serde_json::json!({
        "Error": "Use provided route"
    }))
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.default_service(web::route().to(unmatched));
    cfg.service(web::scope("/api").configure(base_routes));
}
