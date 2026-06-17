use crate::api::base::handler::{app_info, health};
use actix_web::web;

pub fn base_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(app_info));
    cfg.route("/", web::get().to(app_info));
    cfg.route("/health", web::get().to(health));
}
