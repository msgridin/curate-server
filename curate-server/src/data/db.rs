use std::error::Error;
use sqlx::{Executor, Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use std::fs;
use chrono::{DateTime, Utc};
use crate::data::models::{Currency, Rate};
use crate::DBPool;

const DB_POOL_MAX_CONNECTIONS: u32 = 32;
const INIT_SQL: &str = "./db.sql";
const TABLE_CURRENCIES: &str = "currencies";
const TABLE_RATES: &str = "rates";

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

pub(crate) async fn read_currency(currency_id: &str, db_pool: &DBPool) -> Result<Currency, Box<dyn Error>> {
    let currency = sqlx::query_as::<_, Currency>
        (format!("SELECT * FROM {} WHERE id = $1 LIMIT 1", TABLE_CURRENCIES).as_str())
        .bind(currency_id)
        .fetch_one(db_pool).await?;

    Ok(currency)
}

pub(crate) async fn read_currencies(db_pool: &DBPool) -> Result<Vec<Currency>, Box<dyn Error>> {
    let currencies: Vec<Currency> = sqlx::query_as::<_, Currency>(format!("SELECT * FROM {}", TABLE_CURRENCIES).as_str())
        .fetch_all(db_pool).await?;

    Ok(currencies)
}

pub(crate) async fn read_rates(currency: &str, foreign_currency: &str, start_date: DateTime<Utc>, end_date: DateTime<Utc>, db_pool: &DBPool) -> Result<Vec<Rate>, Box<dyn Error>> {
    let rates: Vec<Rate> = sqlx::query_as::<_, Rate>(
        format!("SELECT rate, exchange_date FROM {} WHERE currency = $1 AND foreign_currency = $2 AND exchange_date BETWEEN $3 AND $4", TABLE_RATES).as_str()
    )
        .bind(currency)
        .bind(foreign_currency)
        .bind(start_date)
        .bind(end_date)
        .fetch_all(db_pool).await?;

    Ok(rates)
}

