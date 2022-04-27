use std::collections::HashMap;
use std::error::Error;
use chrono::{Datelike, DateTime, Utc};

pub(crate) async fn get_history_rates(currency_id: &str, date: &DateTime<Utc>, server_url: &str) -> Result<HistoryRatesResponse, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let endpoint = format!("{}{}/{}/{}/{}/{}", server_url, "history", currency_id, date.year(), date.month(), date.day());
    let res = client
        .get(endpoint)
        .send()
        .await?;

    let data = res.json::<HistoryRatesResponse>().await?;

    Ok(data)
}

pub(crate) async fn get_current_rates(currency_id: &str, server_url: &str) -> Result<CurrentRatesResponse, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let endpoint = format!("{}{}/{}", server_url, "latest", currency_id);
    let res = client
        .get(endpoint)
        .send()
        .await?;

    let data = res.json::<CurrentRatesResponse>().await?;

    Ok(data)
}

#[derive(serde_derive::Deserialize, Clone, Debug)]
pub(crate) struct HistoryRatesResponse {
    pub(crate) conversion_rates: HashMap<String, f64>,
}

#[derive(serde_derive::Deserialize, Clone, Debug)]
pub(crate) struct CurrentRatesResponse {
    pub(crate) time_last_update_unix: i64,
    pub(crate) conversion_rates: HashMap<String, f64>,
}

