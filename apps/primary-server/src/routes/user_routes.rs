use axum::{routing::post, Router};
use database::database::DbPool;

use crate::handlers::user_handlers::signup_user;

pub fn user_routes() -> Router<DbPool> {
    Router::new()
    .route("/signup", post(signup_user))
}