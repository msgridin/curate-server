use std::collections::HashMap;
use std::error::Error;
use sqlx::postgres::PgRow;
use sqlx::Row;
use crate::data::models::RateSubscription;
use crate::DBPool;
use crate::data::models::Currency;
use crate::data::db::{TABLE_RATE_SUBSCRIPTIONS, TABLE_CURRENCIES};


pub(crate) async fn read_rate_subscriptions(firebase_token: &str, db_pool: &DBPool) -> Result<Vec<RateSubscription>, Box<dyn Error>> {

    let query = format!("
select
    c1.id,
    c1.name,
    c1.country_id,
    c1.country_name,
    c1.is_crypto,
    c2.id,
    c2.name,
    c2.country_id,
    c2.country_name,
    c2.is_crypto
from
    {0} s
inner join {1} c1 on
    s.from_currency = c1.id
inner join {1} c2 on
    s.to_currency = c2.id
where
    s.firebase_token = $1
order by
    s.from_currency,
    s.to_currency

", TABLE_RATE_SUBSCRIPTIONS, TABLE_CURRENCIES);

    let subscriptions: Vec<RateSubscription> = sqlx::query(query.as_str())
        .bind(firebase_token)
        .map(|row: PgRow| RateSubscription {
            from_currency: Currency {
                id: row.get(0),
                name: row.get(1),
                country_id: row.get(2),
                country_name: row.get(3),
                is_crypto: row.get(4),
                rates: HashMap::new()
            },
            to_currency: Currency {
                id: row.get(5),
                name: row.get(6),
                country_id: row.get(7),
                country_name: row.get(8),
                is_crypto: row.get(9),
                rates: HashMap::new()
            }
        })
        .fetch_all(db_pool).await?;

    Ok(subscriptions)
}

pub(crate) async fn subscribe_rate_notification(from_currency:&str, to_currency: &str, firebase_token: &str, db_pool: &DBPool) -> Result<bool, Box<dyn Error>> {
    let query = format!("INSERT INTO {} (from_currency, to_currency, firebase_token) VALUES ($1, $2, $3) ON CONFLICT (from_currency, to_currency, firebase_token) DO NOTHING", TABLE_RATE_SUBSCRIPTIONS);

    sqlx::query(query.as_str())
        .bind(from_currency)
        .bind(to_currency)
        .bind(firebase_token)
        .execute(db_pool).await?;

    Ok(true)
}

pub(crate) async fn unsubscribe_rate_notification(from_currency:&str, to_currency: &str, firebase_token: &str, db_pool: &DBPool) -> Result<bool, Box<dyn Error>> {
    let query = format!("DELETE FROM {} WHERE from_currency = $1 AND to_currency = $2 AND firebase_token = $3
", TABLE_RATE_SUBSCRIPTIONS);

    sqlx::query(query.as_str())
        .bind(from_currency)
        .bind(to_currency)
        .bind(firebase_token)
        .execute(db_pool).await?;

    Ok(true)
}

