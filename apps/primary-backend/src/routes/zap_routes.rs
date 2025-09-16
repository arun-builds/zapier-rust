use actix_web::web::{ServiceConfig, get, post, route, scope};
use crate::handlers::zap_handlers::{create_zap, get_zap, get_zap_by_id};
use crate::middlewares::auth_middleware::AuthService;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/zap")
        .wrap(AuthService::new())
        .route("/", post().to(create_zap))
        .route("/", get().to(get_zap))
        .route("/{zap_id}", get().to(get_zap_by_id))
    );
}
