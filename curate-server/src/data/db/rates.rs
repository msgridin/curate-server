use std::error::Error;
use chrono::{Datelike, DateTime, Duration, TimeZone, Utc};
use crate::data::models::Rate;
use crate::DBPool;
use crate::data::db::TABLE_RATES;

pub(crate) async fn read_current_rate(currency: &str, foreign_currency: &str, db_pool: &DBPool) -> Result<f64, Box<dyn Error>> {
    let rate: Vec<Rate> = sqlx::query_as::<_, Rate>(
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
        .fetch_all(db_pool).await?;

    match rate.len() {
        0 => Ok(0.0),
        _ => Ok(rate[0].rate)
    }
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
    let mut date = Utc.ymd(start_date.year(), start_date.month(), start_date.day()).and_hms(0, 0, 0);
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

