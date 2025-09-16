use actix_web::web;
use crate::handlers::user_handlers::{get_user};
use actix_web::{HttpResponse, Responder};
use serde_json::json;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
        .route("/health", web::get().to(health_check))
        .route("/{user_id}", web::get().to(get_user))
    );
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(json!({ "message": "User health check successful" }))
}