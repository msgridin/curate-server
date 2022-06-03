use std::collections::HashMap;
use chrono::{Datelike, DateTime, Utc};
use sqlx::Error;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, )]
pub(crate) struct Currency {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) country_id: String,
    pub(crate) country_name: String,
    pub(crate) rates: HashMap<String, CurrentRate>,
    pub(crate) is_crypto: bool,
}

impl<'a, R: ::sqlx::Row> ::sqlx::FromRow<'a, R> for Currency
    where
        &'a ::std::primitive::str: ::sqlx::ColumnIndex<R>,
        String: ::sqlx::decode::Decode<'a, R::Database>,
        String: ::sqlx::types::Type<R::Database>,
        bool: ::sqlx::decode::Decode<'a, R::Database>,
        bool: ::sqlx::types::Type<R::Database>, {
    fn from_row(row: &'a R) -> Result<Self, Error> {
        let id: String = row.try_get("id")?;
        let name: String = row.try_get("name")?;
        let country_id: String = row.try_get("country_id")?;
        let country_name: String = row.try_get("country_name")?;
        let is_crypto: bool = row.try_get("is_crypto")?;
        Result::Ok(Currency { id, name, country_id, country_name, rates: Default::default(), is_crypto })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub(crate) struct GetCurrenciesResponse {
    pub(crate) currencies: Vec<Currency>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub(crate) struct GetCurrencyRatesResponse {
    pub(crate) currency: Currency,
    pub(crate) foreign_currency: Currency,
    pub(crate) current_rates: Vec<Rate>,
    pub(crate) last_week_rates: Vec<Rate>,
    pub(crate) last_month_rates: Vec<Rate>,
    pub(crate) last_year_rates: Vec<Rate>,
    pub(crate) multiplier: i64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub(crate) struct CurrentRate {
    pub(crate) rate: f64,
    pub(crate) multiplier: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub(crate) struct Rate {
    pub(crate) rate: f64,
    pub(crate) exchange_date: DateTime<Utc>,
}

impl PartialEq for Rate {
    fn eq(&self, other: &Self) -> bool {
        self.exchange_date.year() == other.exchange_date.year()
            && self.exchange_date.month() == other.exchange_date.month()
            && self.exchange_date.day() == other.exchange_date.day()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct GetRateNotificationSubscriptionsResponse {
    pub(crate) firebase_token: String,
    pub(crate) subscriptions: Vec<RateSubscription>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, sqlx::FromRow)]
pub(crate) struct RateSubscription {
    pub(crate) from_currency: Currency,
    pub(crate) to_currency: Currency,
}

