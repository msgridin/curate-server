use std::error::Error;
use tokio::time::{Duration, interval};
use sqlx::{Pool, Postgres};

mod data;
mod logic;

pub(crate) type DBPool = Pool<Postgres>;

pub async fn task(main_currencies: [&str; 3], connection_string: &str, currency_remote_url: &str, crypto_remote_url: &str) {

    let db_pool = data::db::init(connection_string).await;

    let mut interval = interval(Duration::from_secs(8 * 3_600));
    loop {
        interval.tick().await;
        let result = run_task(main_currencies, "./currencies.csv", "./crypto.csv", currency_remote_url, crypto_remote_url, &db_pool).await;
        match result {
            Ok(_) => {},
            Err(e) => println!("[DATA IMPORT ERROR] {:?}", e)
        };
    }
}

async fn run_task(main_currencies: [&str; 3], currencies_path: &str, crypto_path: &str, currency_remote_url: &str, crypto_remote_url: &str, db_pool: &DBPool) -> Result<(), Box<dyn Error>>{

    println!("[LOAD CURRENCIES]");
    logic::load_currencies::invoke(currencies_path, crypto_path, db_pool).await?;
    println!("[LOAD RATES]");
    logic::load_rates::invoke(main_currencies, currency_remote_url, crypto_remote_url, db_pool).await?;

    Ok(())
}

