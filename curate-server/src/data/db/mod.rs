use std::error::Error;
use sqlx::{Executor, Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use std::fs;
use crate::DBPool;

pub(crate) const DB_POOL_MAX_CONNECTIONS: u32 = 32;
pub(crate) const INIT_SQL: &str = "./db.sql";
pub(crate) const TABLE_CURRENCIES: &str = "currencies";
pub(crate) const TABLE_RATES: &str = "rates";
pub(crate) const TABLE_RATE_SUBSCRIPTIONS: &str = "rate_subscriptions";

pub(crate) mod currencies;
pub(crate) mod rates;
pub(crate) mod subscriptions;

pub(crate) async fn init(connection_string: &str) -> DBPool {
    let db_pool = create_pool(connection_string).await.expect("ERROR: CREATE DATABASE POOL");
    init_db(&db_pool).await.expect("ERROR: INIT DATABASE");

    db_pool
}

pub async fn create_pool(db_connection_string: &str) -> Result<Pool<Postgres>, Box<dyn Error>> {
    let db_pool = PgPoolOptions::new()
        .max_connections(DB_POOL_MAX_CONNECTIONS)
        .connect(db_connection_string).await?;

    Ok(db_pool)
}

pub(crate) async fn init_db(db_pool: &DBPool) -> Result<(), Box<dyn Error>> {
    let init_file =
        fs::read_to_string(INIT_SQL)
            .map_err(|e| format!("{:?}", e))?;

    db_pool.execute(init_file.as_str()).await?;

    Ok(())
}

