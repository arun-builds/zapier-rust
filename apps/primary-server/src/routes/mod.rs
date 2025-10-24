mod user_routes;
use axum::{routing::get, Router};
use database::database::DbPool;

pub fn v1_router() -> Router<DbPool> {
    Router::new()
    .nest("/v1/user", user_routes::user_routes())
    .route("/health", get(|| async { "/v1 route" }))
   
}