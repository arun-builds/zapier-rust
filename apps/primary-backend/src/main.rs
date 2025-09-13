use axum::Router;

mod routes;
mod handlers;
mod middlewares;

#[tokio::main]
async fn main() {


let app = Router::new()
.nest("/api/v1", routes::app_router());

    println!("Primary backend is running on port 8080");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

