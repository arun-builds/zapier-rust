mod user_routes;
use axum::{Router, routing::get};
use sqlx::PgPool;

pub fn v1_router() -> Router<PgPool> {
    Router::new()
        .nest("/v1/user", user_routes::user_routes())
        .route("/health", get(|| async { "/v1 route" }))
}
