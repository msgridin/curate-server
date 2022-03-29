use std::error::Error;
use crate::data::models::Currency;

pub(crate) fn read_currencies(path: &str) -> Result<Vec<Currency>, Box<dyn Error>> {

    let mut currencies: Vec<Currency> = vec![];

    let mut reader = csv::Reader::from_path(path)?;
    let headers = reader.headers()?;

    for record in reader.deserialize() {
        let currency: Currency = record?;
        currencies.push(currency);
    }

    Ok(currencies)
}
