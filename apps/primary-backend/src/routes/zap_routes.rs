use axum::{
    Router,
    routing::{get, post},
};

use crate::handlers::zap_handlers::{get_zap, create_zap};

// Assuming you have a users handler
// use crate::handlers::users::{list_users};

pub fn zap_routes() -> Router {
    Router::new()
        .route("/", post(create_zap))
        .route("/", get(get_zap))
       
}
