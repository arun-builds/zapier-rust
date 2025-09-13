use axum::{Router, routing::get};


mod user_routes;
mod zap_routes;

// Function to create and compose the main API router
pub fn app_router() -> Router{
    Router::new()
        .nest("/users", user_routes::users_routes())
        .route("/health", get(|| async { "API is healthy!" }))
}