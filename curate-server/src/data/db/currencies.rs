use std::collections::HashMap;
use std::error::Error;
use crate::data::models::{Currency, CurrentRate};
use crate::data::db::rates::read_current_rate;
use crate::DBPool;
use crate::logic::util::get_multiplier;
use crate::data::db::TABLE_CURRENCIES;

pub(crate) async fn read_currencies(db_pool: &DBPool) -> Result<Vec<Currency>, Box<dyn Error>> {
    let cur_list: Vec<Currency> = sqlx::query_as::<_, Currency>(format!("SELECT * FROM {} ORDER BY id", TABLE_CURRENCIES).as_str())
        .fetch_all(db_pool).await?;

    let mut currencies: Vec<Currency> = vec![];

    for cur in &cur_list {
        let usd_rate = read_current_rate("USD", cur.id.as_str(), db_pool).await?;
        let usd_multiplier = get_multiplier(usd_rate);
        let eur_rate = read_current_rate("EUR", cur.id.as_str(), db_pool).await?;
        let eur_multiplier = get_multiplier(eur_rate);
        let uah_rate = read_current_rate("UAH", cur.id.as_str(), db_pool).await?;
        let uah_multiplier = get_multiplier(uah_rate);

        let rates = HashMap::from([
            ("USD".to_string(), CurrentRate {
                rate: usd_rate * (usd_multiplier as f64),
                multiplier: usd_multiplier,
            }),
            ("EUR".to_string(), CurrentRate {
                rate: eur_rate * (eur_multiplier as f64),
                multiplier: eur_multiplier,
            }),
            ("UAH".to_string(), CurrentRate {
                rate: uah_rate * (uah_multiplier as f64),
                multiplier: uah_multiplier,
            }),
        ]);

        currencies.push(Currency {
            id: cur.id.clone(),
            name: cur.name.clone(),
            country_id: cur.country_id.clone(),
            country_name: cur.country_name.clone(),
            is_crypto: cur.is_crypto,
            rates
        });
    }
    Ok(currencies)
}

pub(crate) async fn read_currency(currency_id: &str, db_pool: &DBPool) -> Result<Currency, Box<dyn Error>> {
    let currency = sqlx::query_as::<_, Currency>
        (format!("SELECT * FROM {} WHERE id = $1 LIMIT 1", TABLE_CURRENCIES).as_str())
        .bind(currency_id)
        .fetch_one(db_pool).await?;

    Ok(currency)
}
