use std::error::Error;
use chrono::{DateTime, TimeZone, Utc};
use crate::DBPool;
use crate::data;
use csv;
use crate::data::db::{read_currency, read_last_rate};

pub(crate) async fn invoke(db_pool: &DBPool) -> Result<(), Box<dyn Error>> {

    println!("LOADING RATES");

    const CURRENCIES: [&str; 2] = ["USD", "EUR"];

    for currency in CURRENCIES {
        let mut date = Utc.ymd(2021, 1, 1).and_hms(0, 0, 0);
        date = read_last_rate(currency, &db_pool).await
            .map(|rate| rate.date)
            .map_err(|e| {
                println!("{:?}", e);
                e.to_string()
            })
            .unwrap_or(date);

        let end_date = Utc::now();
        while date < end_date {
            println!("{}", date.format("%Y-%m-%d"));
            let rates = data::api::get_rates(currency, Some(&date)).await?;
            for (foreign_currency, rate) in rates {
                data::db::create_rate(currency, foreign_currency.as_str(), rate, date, db_pool).await?;
            }
            date = date.checked_add_signed(chrono::Duration::days(1)).unwrap();
        }
    }

    Ok(())
}
