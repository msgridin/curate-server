use std::error::Error;
use tokio::time::{Duration, interval};
use sqlx::{Pool, Postgres};

mod data;
mod logic;

pub(crate) type DBPool = Pool<Postgres>;

pub async fn task(main_currencies: [&str; 3], server_api_key: &str, connection_string: &str) {

    let db_pool = data::db::init(connection_string).await;

    let mut interval = interval(Duration::from_secs(8 * 3_600));
    loop {
        interval.tick().await;
        let result = run_task(main_currencies, server_api_key: &str, &db_pool).await;
        match result {
            Ok(_) => {},
            Err(e) => println!("{:?}", e)
        };
    }
}

async fn run_task(main_currencies: [&str; 3], server_api_key: &str, db_pool: &DBPool) -> Result<(), Box<dyn Error>> {

    Ok(())
}

