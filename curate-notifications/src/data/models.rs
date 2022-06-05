#![allow(dead_code)]
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, serde::Deserialize, sqlx::FromRow)]
pub(crate) struct Rate {
    pub(crate) currency: String,
    pub(crate) foreign_currency: String,
    pub(crate) rate: f64,
    pub(crate) exchange_date: DateTime<Utc>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, sqlx::FromRow)]
pub(crate) struct RateSubscription {
    pub(crate) from_currency: String,
    pub(crate) to_currency: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub(crate) struct CurrentRate {
    pub(crate) from_currency: String,
    pub(crate) to_currency: String,
    pub(crate) rate: f64,
    pub(crate) multiplier: i64,
}

