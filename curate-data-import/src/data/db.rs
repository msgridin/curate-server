use std::error::Error;
use sqlx::{Executor, Pool, Postgres, Row};
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

pub(crate) async fn create_currency(currency: Currency, db_pool: &DBPool) -> Result<(), Box<dyn Error>> {
    sqlx::query(format!("INSERT INTO {}
    (id, name, country_id, country_name, is_crypto) VALUES ($1, $2, $3, $4, $5)
    ON CONFLICT (id) DO UPDATE SET name = $2, country_id = $3, country_name = $4, is_crypto = $5", TABLE_CURRENCIES).as_str())
        .bind(&currency.id)
        .bind(&currency.name)
        .bind(&currency.country_id)
        .bind(&currency.country_name)
        .bind(&currency.is_crypto)
        .execute(db_pool).await?;

    Ok(())
}

pub(crate) async fn create_rate(currency: &str, foreign_currency: &str, rate: f64, date: DateTime<Utc>, db_pool: &DBPool) -> Result<(), Box<dyn Error>> {

    sqlx::query(format!("INSERT INTO {}
    (currency, foreign_currency, rate, exchange_date) VALUES ($1, $2, $3, $4)
    ON CONFLICT (currency, foreign_currency, exchange_date) DO UPDATE SET foreign_currency = $2, rate = $3, exchange_date = $4", TABLE_RATES).as_str())
        .bind(currency)
        .bind(foreign_currency)
        .bind(rate)
        .bind(date)
        .execute(db_pool).await?;

    Ok(())
}

pub(crate) async fn read_last_rate(currency_id: &str, is_crypto: bool, db_pool: &DBPool) -> Result<Rate, Box<dyn Error>> {

    let query = format!("
    select
        c.id,
        c.name,
        c.country_id,
        c.country_name,
        c2.id,
        c2.name,
        c2.country_id,
        c2.country_name,
        r.rate,
        r.exchange_date,
        c.is_crypto,
        c2.is_crypto
    from
        {} r
    inner join {} c on
        r.currency = c.id
    inner join currencies c2 on
        r.foreign_currency = c2.id
    where
        r.currency = $1 and r.foreign_currency = $2
    order by
        r.exchange_date desc
    limit 1", TABLE_RATES, TABLE_CURRENCIES);
    let res = sqlx::query(query.as_str())
        .bind(currency_id)
        .bind(if is_crypto { "BTC" } else { "GBP" })
        .fetch_one(db_pool)
        .await?;

    Ok(Rate {
            currency: Currency {
                id: res.get(0),
                name: res.get(1),
                country_id: res.get(2),
                country_name: res.get(3),
                is_crypto: res.get(10),
            },
            foreign_currency: Currency {
                id: res.get(4),
                name: res.get(5),
                country_id: res.get(6),
                country_name: res.get(7),
                is_crypto: res.get(11),
            },
            rate: res.get(8),
            date: res.get(9)
        })
}