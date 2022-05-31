use std::error::Error;
use chrono::{Datelike, DateTime, NaiveDateTime, TimeZone, Utc};
use crate::DBPool;
use crate::data;

pub(crate) async fn invoke(main_currencies: [&str; 3], currency_remote_url: &str, crypto_remote_url: &str, db_pool: &DBPool) -> Result<(), Box<dyn Error>> {
    println!("LOADING CURRENCY RATES");
    load_historical_rates(main_currencies, false, currency_remote_url, crypto_remote_url, db_pool).await?;
    load_current_rates(main_currencies, false, currency_remote_url, crypto_remote_url, db_pool).await?;

    println!("LOADING CRYPTO RATES");
    load_historical_rates(main_currencies, true, currency_remote_url, crypto_remote_url, db_pool).await?;
    load_current_rates(main_currencies, true, currency_remote_url, crypto_remote_url, db_pool).await?;

    Ok(())
}

async fn load_historical_rates(main_currencies: [&str; 3], is_crypto: bool, currency_remote_url: &str, crypto_remote_url: &str, db_pool: &DBPool) -> Result<(), Box<dyn Error>> {
    for currency in main_currencies {
        let mut date = Utc.ymd(2021, 1, 1).and_hms(0, 0, 0);
        date = data::db::read_last_rate(currency, is_crypto, db_pool).await
            .map(|rate| rate.date)
            .map(|date| Utc.ymd(date.year(), date.month(), date.day())
                .and_hms(0, 0, 0))
            .unwrap_or(date);
        date = date.checked_add_signed(chrono::Duration::days(1)).unwrap();

        let now = Utc::now();
        let end_date = Utc.ymd(now.year(), now.month(), now.day()).and_hms(0, 0, 0)
            .checked_sub_signed(chrono::Duration::days(1)).unwrap();
        while date <= end_date {
            // Get history rates
            println!("{}: {}", currency, date.format("%Y-%m-%d"));
            let rates = if is_crypto {
                data::api::get_crypto_history_rates(currency, &date, crypto_remote_url).await?
            } else {
                data::api::get_history_rates(currency, &date, currency_remote_url).await?
            };
            for (foreign_currency, rate) in rates.conversion_rates {
                data::db::create_rate(currency, foreign_currency.as_str(), rate, date, db_pool).await?;
            };
            date = date.checked_add_signed(chrono::Duration::days(1)).unwrap();
        }
    }

    Ok(())
}

async fn load_current_rates(main_currencies: [&str; 3], is_crypto: bool, currency_remote_url: &str, crypto_remote_url: &str, db_pool: &DBPool) -> Result<(), Box<dyn Error>> {
    for currency in main_currencies {
        // Get current rates
        let rates = if is_crypto {
            data::api::get_crypto_current_rates(currency, crypto_remote_url).await?
        } else {
            data::api::get_current_rates(currency, currency_remote_url).await?
        };
        let mut date = DateTime::from_utc(NaiveDateTime::from_timestamp(rates.time_last_update_unix, 0), Utc);
        date = Utc.ymd(date.year(), date.month(), date.day()).and_hms(0, 0, 0);
        println!("{}: {}", currency, date.format("%Y-%m-%d"));
        for (foreign_currency, rate) in rates.conversion_rates {
            data::db::create_rate(currency, foreign_currency.as_str(), rate, date, db_pool).await?;
        };
    };

    Ok(())
}