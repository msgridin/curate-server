use std::error::Error;
use sqlx::{ Executor, Pool, Postgres };
use sqlx::postgres::PgPoolOptions;
use std::fs;
use crate::DBPool;

const DB_POOL_MAX_CONNECTIONS: u32 = 32;
const INIT_SQL: &str = "./db.sql";
const TABLE_CURRENCIES: &str = "currencies";
const TABLE_RATES: &str = "rates";

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
