use chrono::{Datelike, DateTime, Utc};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub(crate) struct Currency {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) country_id: String,
    pub(crate) country_name: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub(crate) struct GetCurrenciesResponse {
    pub(crate) currencies: Vec<Currency>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub(crate) struct Rate {
    pub(crate) rate: f64,
    pub(crate) exchange_date: DateTime<Utc>
}

impl PartialEq for Rate {
    fn eq(&self, other: &Self) -> bool {
        self.exchange_date.year() == other.exchange_date.year()
            && self.exchange_date.month() == other.exchange_date.month()
            && self.exchange_date.day() == other.exchange_date.day()
    }
}