use dotenvy::dotenv;
use std::env;
use diesel::{r2d2::{self, ConnectionManager}, PgConnection};

pub struct Config {
    pub pool: r2d2::Pool<ConnectionManager<PgConnection>>,
}

impl Default for Config {
    fn default() -> Self {
        match dotenv() {
            Ok(_) => println!("Loaded .env file"),
            Err(e) => println!("Could not load .env file: {}", e),
        }

        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| panic!("Please provide the database_url environment variable"));

let manager = ConnectionManager::<PgConnection>::new(database_url);

let pool = r2d2::Pool::builder()
.build(manager)
.expect("Failed to create pool");
        
        Self { pool }
    }
}
