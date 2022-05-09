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

pub(crate) async fn get_crypto_history_rates(currency_id: &str, date: &DateTime<Utc>, server_url: &str) -> Result<HistoryRatesResponse, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let server_url = server_url.replace("<TYPE>", date.format("%Y-%m-%d").to_string().as_str());
    let endpoint = format!("{}&target={}", server_url, currency_id);
    let res = client
        .get(endpoint)
        .send()
        .await?;

    let data = res.json::<CryptoHistoryRatesResponse>().await?;
    let data = HistoryRatesResponse {
        conversion_rates: invert_rates(&data.rates),
    };

    Ok(data)
}

pub(crate) async fn get_crypto_current_rates(currency_id: &str, server_url: &str) -> Result<CurrentRatesResponse, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let server_url = server_url.replace("<TYPE>", "live");
    let endpoint = format!("{}&target={}", server_url, currency_id);
    let res = client
        .get(endpoint)
        .send()
        .await?;

    let data = res.json::<CryptoCurrentRatesResponse>().await?;
    let data = CurrentRatesResponse {
        time_last_update_unix: data.timestamp,
        conversion_rates: invert_rates(&data.rates),
    };

    Ok(data)
}

fn invert_rates(rates: &HashMap<String, f64>) -> HashMap<String, f64> {
    let mut inverted_rates: HashMap<String, f64> = HashMap::new();
    for (key, value) in rates.iter() {
        inverted_rates.insert(key.to_string(), if value == &0.0 { 0.0 } else { 1.0 / value });
    }

    inverted_rates
}

#[derive(serde_derive::Deserialize, Clone, Debug)]
pub(crate) struct CryptoHistoryRatesResponse {
    pub(crate) rates: HashMap<String, f64>,
}

#[derive(serde_derive::Deserialize, Clone, Debug)]
pub(crate) struct CryptoCurrentRatesResponse {
    pub(crate) timestamp: i64,
    pub(crate) rates: HashMap<String, f64>,
}

