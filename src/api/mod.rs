pub mod auth;
pub mod base;
use crate::api::base::routes::base_routes;
use actix_web::{HttpResponse, Responder, web};

pub async fn unmatched() -> impl Responder {
    HttpResponse::NotFound().json(serde_json::json!({
        "Error": "Use provided route"
    }))
}

pub async fn favicon() -> impl Responder {
    HttpResponse::NoContent().finish()
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.default_service(web::route().to(unmatched));
    cfg.route("/favicon.ico", web::get().to(favicon));
    cfg.route("/api/routes", web::get().to(all_routes));
    cfg.service(web::scope("/api").configure(base_routes));
}

pub async fn all_routes() -> impl Responder {
    web::Json(serde_json::json!({
        "routes": [
            {
                "path": "/api",
                "description": "Base routes"
            },
            {
                "path": "/api/routes",
                "description": "List all routes"
            },
            {
                "path": "/api/auth",
                "description": "Auth routes"
            },
            {
                "path": "/api/",
                "description": ""
            }
        ]
    }))
}
