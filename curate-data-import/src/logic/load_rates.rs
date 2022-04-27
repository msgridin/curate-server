use std::error::Error;
use chrono::{Datelike, DateTime, NaiveDateTime, TimeZone, Utc};
use crate::DBPool;
use crate::data;

pub(crate) async fn invoke(server_url: &str, db_pool: &DBPool) -> Result<(), Box<dyn Error>> {

    println!("LOADING RATES");

    const CURRENCIES: [&str; 3] = ["USD", "EUR", "UAH"];

    for currency in CURRENCIES {
        let mut date = Utc.ymd(2021, 1, 1).and_hms(0, 0, 0);
        date = data::db::read_last_rate(currency, db_pool).await
            .map(|rate| rate.date)
            .unwrap_or(date);

        let now = Utc::now();
        let end_date = Utc.ymd(now.year(), now.month(), now.day()).and_hms(0, 0, 0);
        while date < end_date {
            // Get history rates
            println!("{}", date.format("%Y-%m-%d"));
            let rates = data::api::get_history_rates(currency, &date, server_url).await?;
            for (foreign_currency, rate) in rates.conversion_rates {
                data::db::create_rate(currency, foreign_currency.as_str(), rate, date, db_pool).await?;
            };
            date = date.checked_add_signed(chrono::Duration::days(1)).unwrap();
        }

        // Get current rates
        println!("{}", date.format("%Y-%m-%d"));
        let rates = data::api::get_current_rates(currency, server_url).await?;
        date = DateTime::from_utc(NaiveDateTime::from_timestamp(rates.time_last_update_unix, 0), Utc);

        for (foreign_currency, rate) in rates.conversion_rates {
            data::db::create_rate(currency, foreign_currency.as_str(), rate, date, db_pool).await?;
        };

    }

    Ok(())
}

