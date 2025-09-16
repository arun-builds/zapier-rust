use actix_web::{web, App, HttpServer, middleware, http::header};
use actix_cors::Cors;
use std::io;
use routes::{auth_routes::config as auth_config, user_routes::config as user_config};
use database::establish_connection_pool;

mod routes;
mod handlers;
mod types;
mod utils;

#[actix_web::main]
async fn main() -> io::Result<()> {
    
    
    // Create a shared database connection
    let pool = web::Data::new(establish_connection_pool());

    log::info!("Starting server at http://localhost:8080");
    println!("Starting server at http://localhost:8080");

    HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")  // Your frontend URL
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE,
            ])
            .supports_credentials()
            .max_age(86400);

        App::new()
            .app_data(pool.clone())
            .wrap(cors) 
            .wrap(middleware::Logger::default())
            .wrap(middleware::DefaultHeaders::new().add(("X-Version", "1.0")))
            .service(
                web::scope("/api/v1")
                    .configure(auth_config)
                    .configure(user_config)
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}