use actix_web::web;
use crate::handlers::user_handlers::{signup_user, login_user, get_user};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
        .route("/signup", web::post().to(signup_user))
        .route("/login", web::post().to(login_user))
        .route("/get", web::get().to(get_user))
    );
}