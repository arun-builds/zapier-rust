use crate::config::Config;
use diesel::prelude::*;

pub struct Database {
    pub conn: PgConnection,
}

impl Database {
    pub fn new() -> Result<Self, ConnectionError> {
        let config = Config::default();
        let conn = PgConnection::establish(&config.database_url)?;
        Ok(Self { conn })
    }
}
