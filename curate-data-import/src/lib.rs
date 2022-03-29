use std::error::Error;
use std::future::Future;
use tokio::time::{Duration, interval};
use sqlx::{Pool, Postgres};

mod data;
mod logic;

pub(crate) type DBPool = Pool<Postgres>;

pub async fn task(db_connection_string: &str) {
    let db_pool = data::db::create_pool(db_connection_string).await.expect("ERROR: CAN NOT CREATE DATABASE POOL");

    let mut interval = interval(Duration::from_secs(60));
    loop {
        interval.tick().await;
        let result = run_task("./currencies.csv", &db_pool).await;
        match result {
            Ok(_) => {},
            Err(e) => println!("{:?}", e)
        };
    }
}

async fn run_task(currencies_path: &str, db_pool: &DBPool) -> Result<(), Box<dyn Error>>{
    logic::load_currencies::invoke(currencies_path, db_pool).await?;
    logic::load_rates::invoke(&db_pool).await?;

    Ok(())
}

