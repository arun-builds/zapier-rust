use actix_web::web;
use crate::handlers::auth_handlers::{signup_user, login_user, logout};
use actix_web::{HttpResponse, Responder};
use serde_json::json;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
        .route("/health", web::get().to(health_check))
        .route("/signup", web::post().to(signup_user))
        .route("/login", web::post().to(login_user))
        .route("/logout", web::post().to(logout))
    );
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(json!({ "message": "Auth health check successful" }))
}