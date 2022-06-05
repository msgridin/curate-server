#![allow(dead_code)]
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, serde::Deserialize, sqlx::FromRow)]
pub(crate) struct Rate {
    pub(crate) currency: String,
    pub(crate) foreign_currency: String,
    pub(crate) rate: f64,
    pub(crate) date: DateTime<Utc>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, sqlx::FromRow)]
pub(crate) struct RateSubscription {
    pub(crate) from_currency_id: String,
    pub(crate) to_currency_id: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub(crate) struct CurrentRate {
    pub(crate) from_currency_id: String,
    pub(crate) to_currency_id: String,
    pub(crate) rate: f64,
    pub(crate) multiplier: i64,
}

