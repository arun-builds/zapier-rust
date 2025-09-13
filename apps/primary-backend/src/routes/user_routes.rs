use axum::{
    Router,
    routing::{get, post},
    middleware,
};

use crate::handlers::user_handlers::{signup_user, login_user, get_user};
use crate::middlewares::auth_middleware;

// Assuming you have a users handler
// use crate::handlers::users::{list_users};

pub fn users_routes() -> Router {
    let protected_routes = Router::new()
        .route("/signup", post(signup_user))
        .route("/login", post(login_user))
        .layer(middleware::from_fn(auth_middleware));

    Router::new()
    .route("/", get(get_user))
    .merge(protected_routes)


       
}
