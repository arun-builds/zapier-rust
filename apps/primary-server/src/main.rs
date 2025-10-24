use axum::Router;
use database::config::Config;
use sqlx::postgres::PgPoolOptions;

mod errors;
mod handlers;
mod routes;
mod types;

#[tokio::main]
async fn main() {
    let config = Config::default();
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("can't connect to database");

    let app = Router::new()
        .nest("/api", routes::v1_router())
        .with_state(db_pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
