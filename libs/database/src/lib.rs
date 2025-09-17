use diesel::{pg::PgConnection, r2d2::{self, ConnectionManager}};
use std::env;
use dotenvy::dotenv;
pub mod models;
pub mod schema;


pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection_pool() -> DbPool {
    match dotenv() {
        Ok(_) => println!("Loaded .env file"),
        Err(e) => println!("Could not load .env file: {}", e),
    }

    let database_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(e) => panic!("DATABASE_URL not found in environment: {}", e),
    };

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}