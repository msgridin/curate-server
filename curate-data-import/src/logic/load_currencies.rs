use std::error::Error;
use chrono::{DateTime, TimeZone, Utc};
use crate::DBPool;
use crate::data;
use csv;
use crate::data::db::{read_currency, read_last_rate};

pub(crate) async fn invoke(currencies_path: &str, db_pool: &DBPool) -> Result<(), Box<dyn Error>> {

    println!("LOADING CURRENCIES");

    let currencies = data::fs::read_currencies(currencies_path)?;

    for currency in currencies {
        data::db::create_currency(currency, db_pool).await?;
    }

    Ok(())
}
