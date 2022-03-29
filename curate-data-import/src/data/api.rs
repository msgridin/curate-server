use std::collections::HashMap;
use std::error::Error;
use chrono::{Datelike, DateTime, Utc};
use crate::data::models::Currency;

const SOURCE_SERVER_URL: &str = "https://v6.exchangerate-api.com/v6/76d3420366f5bbcc4f8096de/";

pub(crate) async fn get_rates(currency_id: &str, date: Option<&DateTime<Utc>>) -> Result<HashMap<String, f64>, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let endpoint = match date {
        None => format!("{}{}/{}", SOURCE_SERVER_URL, "latest", currency_id),
        Some(date) => format!("{}{}/{}/{}/{}/{}", SOURCE_SERVER_URL, "history", currency_id, date.year(), date.month(), date.day())
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
    pub(crate) base_code: String,
    pub(crate) conversion_rates: HashMap<String, f64>,
}

