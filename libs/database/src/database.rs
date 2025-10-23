use deadpool_diesel::{postgres::BuildError, Manager, Pool};
use crate::config::Config;
use diesel::{pg::PgConnection};

pub type DbPool = Pool<Manager<PgConnection>>;




pub fn create_pool() -> Result<DbPool, BuildError> {
    let config = Config::default();
    let manager = deadpool_diesel::postgres::Manager::new(
        &config.database_url,
        deadpool_diesel::Runtime::Tokio1,
    );
    deadpool_diesel::postgres::Pool::builder(manager).build()
}
