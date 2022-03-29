use chrono::{Datelike, DateTime, Timelike, TimeZone, Utc};
use warp::{reject, Rejection, Reply};
use crate::{data, DBPool};
use crate::data::models::{Currency, Rate};
use crate::error::ServerError;

pub(crate) async fn invoke(currency_id: String, foreign_currency_id: String, db_pool: DBPool) -> Result<impl Reply, Rejection> {
    let currency = data::db::read_currency(currency_id.as_str(), &db_pool).await
        .map_err(|e| reject::custom(ServerError::from(e)))?;

    let foreign_currency = data::db::read_currency(foreign_currency_id.as_str(), &db_pool).await
        .map_err(|e| reject::custom(ServerError::from(e)))?;


    let now = Utc::now();
    let date = now;
    let current = get_rates(currency_id.as_str(), foreign_currency_id.as_str(), date, &db_pool).await?;

    let date = now.checked_sub_signed(chrono::Duration::days(7)).unwrap();
    let last_week = get_rates(currency_id.as_str(), foreign_currency_id.as_str(), date, &db_pool).await?;

    let date = dec_month(now);
    let last_month = get_rates(currency_id.as_str(), foreign_currency_id.as_str(), date, &db_pool).await?;

    let date = dec_year(now);
    let last_year = get_rates(currency_id.as_str(), foreign_currency_id.as_str(), date, &db_pool).await?;

    let rates: GetCurrencyRatesRequest = GetCurrencyRatesRequest {
        currency,
        foreign_currency,
        current,
        last_week,
        last_month,
        last_year
    };

    let json = serde_json::to_string(&rates)
        .map_err(|e| reject::custom(ServerError(format!("{:?}", e))))?;

    Ok(json)
}

fn dec_month(date: DateTime<Utc>) -> DateTime<Utc> {
    let year = match date.month() {
        1 => date.year() - 1,
        _ => date.year(),
    };

    let month = match date.month() {
        1 => 12,
        _ => date.month() - 1,
    };

    Utc.ymd(year, month, date.day()).and_hms(date.hour(), date.minute(), date.second())
}

fn dec_year(date: DateTime<Utc>) -> DateTime<Utc> {
    Utc.ymd(date.year() - 1, date.month(), date.day()).and_hms(date.hour(), date.minute(), date.second())
}

async fn get_rates(currency_id: &str, foreign_currency_id: &str, end_date: DateTime<Utc>, db_pool: &DBPool) -> Result<Vec<Rate>, Rejection> {
    let start_date = dec_year(end_date);
    let rates = data::db::read_rates(currency_id, foreign_currency_id, start_date, end_date, db_pool).await
        .map_err(|e| reject::custom(ServerError::from(e)))?;

    Ok(rates)
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct GetCurrencyRatesRequest {
    currency: Currency,
    foreign_currency: Currency,
    current: Vec<Rate>,
    last_week: Vec<Rate>,
    last_month: Vec<Rate>,
    last_year: Vec<Rate>,
}