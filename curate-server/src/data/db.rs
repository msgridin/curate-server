use std::collections::HashMap;
use std::error::Error;
use sqlx::{Executor, Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use std::fs;
use chrono::{Datelike, DateTime, Duration, TimeZone, Utc};
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
    let cur_list: Vec<Currency> = sqlx::query_as::<_, Currency>(format!("SELECT * FROM {} ORDER BY id", TABLE_CURRENCIES).as_str())
        .fetch_all(db_pool).await?;

    let mut currencies: Vec<Currency> = vec![];

    for cur in &cur_list {
        let usd = read_current_rate("USD", cur.id.as_str(), db_pool).await?;
        let eur = read_current_rate("EUR", cur.id.as_str(), db_pool).await?;
        let uah = read_current_rate("UAH", cur.id.as_str(), db_pool).await?;

        let rates = HashMap::from([
            ("USD".to_string(), usd.rate),
            ("EUR".to_string(), eur.rate),
            ("UAH".to_string(), uah.rate),
        ]);

        currencies.push(Currency {
            id: cur.id.clone(),
            name: cur.name.clone(),
            country_id: cur.country_id.clone(),
            country_name: cur.country_name.clone(),
            rates
        });
    }

    Ok(currencies)
}

pub(crate) async fn read_rates(currency: &str, foreign_currency: &str, start_date: DateTime<Utc>, end_date: DateTime<Utc>, db_pool: &DBPool) -> Result<Vec<Rate>, Box<dyn Error>> {
    let db_rates: Vec<Rate> = sqlx::query_as::<_, Rate>(
        format!("SELECT rate, exchange_date FROM {} WHERE currency = $1 AND foreign_currency = $2 AND exchange_date BETWEEN $3 AND $4", TABLE_RATES).as_str()
    )
        .bind(currency)
        .bind(foreign_currency)
        .bind(start_date)
        .bind(end_date)
        .fetch_all(db_pool).await?;

    let mut rates: Vec<Rate> = Vec::new();
    let mut temp_rate = 0.0;
    let mut date = Utc.ymd(start_date.year(), start_date.month(), start_date.day()+1).and_hms(0, 0, 0);
    while date <= end_date {
        let position = db_rates.iter().position(|r| r.exchange_date == date);
        temp_rate = match position {
            Some(p) => {
                temp_rate = db_rates[p].rate;
                temp_rate
            }
            None => temp_rate
        };

        rates.push(Rate {
            rate: temp_rate,
            exchange_date: date
        });

        date = date.checked_add_signed(Duration::days(1)).unwrap();
    }

    Ok(rates)
}

pub(crate) async fn read_current_rate(currency: &str, foreign_currency: &str, db_pool: &DBPool) -> Result<Rate, Box<dyn Error>> {
    let rate: Rate = sqlx::query_as::<_, Rate>(
        format!("
select
    r.currency,
    r.foreign_currency,
    r.exchange_date,
    r.rate
from
    {} r
where
    r.currency = $1
    and r.foreign_currency = $2
order by
    r.exchange_date desc
limit 1", TABLE_RATES).as_str()
    )
        .bind(currency)
        .bind(foreign_currency)
        .fetch_one(db_pool).await?;

    Ok(rate)
}

