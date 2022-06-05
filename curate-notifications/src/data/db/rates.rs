use std::error::Error;
use sqlx::postgres::PgRow;
use sqlx::Row;
use crate::data::db::{TABLE_RATE_SUBSCRIPTIONS, TABLE_RATES};
use crate::data::models::{Rate, RateSubscription};
use crate::DBPool;

pub(crate) async fn read_current_rate(currency: &str, foreign_currency: &str, db_pool: &DBPool) -> Result<f64, Box<dyn Error>> {
    let rate: Vec<Rate> = sqlx::query_as::<_, Rate>(
        format!("
select
    r.currency,
    r.foreign_currency,
    r.rate,
    r.exchange_date
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
        .fetch_all(db_pool).await?;

    match rate.len() {
        0 => Ok(0.0),
        _ => Ok(rate[0].rate)
    }
}

pub(crate) async fn read_firebase_tokens(db_pool: &DBPool) -> Result<Vec<String>, Box<dyn Error>> {
    let tokens: Vec<String> = sqlx::query(
        format!("select distinct r.firebase_token from {} r", TABLE_RATE_SUBSCRIPTIONS).as_str()
    )
        .map(|row: PgRow| row.get(0))
        .fetch_all(db_pool).await?;

    Ok(tokens)
}

pub(crate) async fn read_rate_subscriptions(firebase_token: &str, db_pool: &DBPool) -> Result<Vec<RateSubscription>, Box<dyn Error>> {
    let subscriptions: Vec<RateSubscription> = sqlx::query_as::<_, RateSubscription>(
        format!("
        select
            r.from_currency ,
            r.to_currency,
            r.firebase_token
        from
            {} r
        where
            r.firebase_token = $1

        ", TABLE_RATE_SUBSCRIPTIONS).as_str()
    )
        .bind(firebase_token)
        .fetch_all(db_pool).await?;

    Ok(subscriptions)
}
