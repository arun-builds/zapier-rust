use actix_web::web;
use crate::handlers::user_handlers::{get_user};
use actix_web::{HttpResponse, Responder};
use serde_json::json;
use crate::middlewares::auth_middleware::AuthService;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
        .wrap(AuthService::new())
        .route("/health", web::get().to(health_check))
        .route("/", web::get().to(get_user))
    );
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(json!({ "message": "User health check successful" }))
}