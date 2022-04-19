use std::collections::HashMap;
use std::error::Error;
use chrono::{Datelike, DateTime, Utc};

pub(crate) async fn get_rates(currency_id: &str, date: Option<&DateTime<Utc>>, server_url: &str) -> Result<HashMap<String, f64>, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let endpoint = match date {
        None => format!("{}{}/{}", server_url, "latest", currency_id),
        Some(date) => format!("{}{}/{}/{}/{}/{}", server_url, "history", currency_id, date.year(), date.month(), date.day())
    };
    let res = client
        .get(endpoint)
        .send()
        .await?;

    let data = res.json::<RatesResponse>().await?;

    Ok(data.conversion_rates)
}

#[derive(serde_derive::Deserialize, Clone, Debug)]
pub(crate) struct RatesResponse {
    pub(crate) conversion_rates: HashMap<String, f64>,
}

