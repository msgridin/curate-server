use chrono::{DateTime, Utc};
use sqlx::Row;

#[derive(Debug, Clone, serde::Deserialize, sqlx::FromRow)]
pub(crate) struct Currency {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) country_id: String,
    pub(crate) country_name: String
}

#[derive(Debug, Clone, serde::Deserialize, sqlx::FromRow)]
pub(crate) struct Rate {
    pub(crate) currency: Currency,
    pub(crate) foreign_currency: Currency,
    pub(crate) rate: f64,
    pub(crate) date: DateTime<Utc>
}

