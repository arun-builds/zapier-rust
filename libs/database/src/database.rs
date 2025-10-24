use sqlx::PgPool;

use crate::config::Config;

pub async fn database() -> PgPool {
    let config = Config::default();
    let pool = sqlx::postgres::PgPool::connect(&config.database_url)
        .await
        .unwrap();
    pool
}
