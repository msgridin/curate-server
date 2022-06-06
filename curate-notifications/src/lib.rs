use std::error::Error;
use chrono::{Timelike, Utc};
use tokio::time::{Duration, interval};
use sqlx::{Pool, Postgres};

mod data;
mod logic;

pub(crate) type DBPool = Pool<Postgres>;

pub async fn task(main_currencies: [&str; 3], server_api_key: &str, connection_string: &str) {

    let db_pool = data::db::init(connection_string).await;

    let mut interval = interval(Duration::from_secs(3_600));
    loop {
        interval.tick().await;
        let now = Utc::now();
        if now.hour() >= 12 && now.hour() < 13 {
            let result = run_task(main_currencies, server_api_key, &db_pool).await;
            match result {
                Ok(_) => {},
                Err(e) => println!("[SEND NOTIFICATIONS ERROR]{:?}", e)
            };
        }
    }
}

async fn run_task(main_currencies: [&str; 3], server_api_key: &str, db_pool: &DBPool) -> Result<(), Box<dyn Error>> {

    println!("[SEND NOTIFICATIONS]");
    logic::send_notifications(main_currencies, server_api_key, &db_pool).await?;
    Ok(())
}

