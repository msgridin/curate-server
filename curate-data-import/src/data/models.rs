#![allow(dead_code)]
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, serde::Deserialize, sqlx::FromRow)]
pub(crate) struct Currency {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) country_id: String,
    pub(crate) country_name: String,
    pub(crate) is_crypto: bool,
}

#[derive(Debug, Clone, serde::Deserialize, sqlx::FromRow)]
pub(crate) struct Rate {
    pub(crate) currency: Currency,
    pub(crate) foreign_currency: Currency,
    pub(crate) rate: f64,
    pub(crate) date: DateTime<Utc>,
}

