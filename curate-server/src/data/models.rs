use chrono::{DateTime, Utc};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub(crate) struct Currency {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) country_id: String,
    pub(crate) country_name: String
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub(crate) struct Rate {
    pub(crate) rate: f64,
    pub(crate) exchange_date: DateTime<Utc>
}

