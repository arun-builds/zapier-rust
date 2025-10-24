use axum::{Router, routing::post};

use crate::handlers::user_handlers::{signin_user, signup_user};

pub fn user_routes() -> Router<sqlx::PgPool> {
    Router::new()
        .route("/signup", post(signup_user))
        .route("/signin", post(signin_user))
}
