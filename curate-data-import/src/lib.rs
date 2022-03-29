use std::error::Error;
use tokio::time::{Duration, interval};
use sqlx::{Pool, Postgres};

mod data;
mod logic;
mod config;

pub(crate) type DBPool = Pool<Postgres>;

pub async fn task(connection_string: &str) {

    let db_pool = data::db::init(connection_string).await;

    let mut interval = interval(Duration::from_secs(8 * 3_600));
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

