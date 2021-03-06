use chrono::Utc;
use sqlx::{Pool, Postgres};
use crate::config::{ENVIRONMENT, SERVER_REST_API_PORT, SERVER_FIREBASE_API_KEY, DATABASE_CONNECTION_STRING, CURRENCY_DATASOURCE_URL, CRYPTO_DATASOURCE_URL, MAIN_CURRENCIES};

mod config;
mod data;
mod router;
mod logic;
mod error;

pub(crate) type DBPool = Pool<Postgres>;

#[tokio::main]
async fn main() {
    greetings();

    let db_pool = data::db::init(DATABASE_CONNECTION_STRING).await;

    tokio::spawn(curate_data_import::task(MAIN_CURRENCIES, DATABASE_CONNECTION_STRING, CURRENCY_DATASOURCE_URL, CRYPTO_DATASOURCE_URL));
    tokio::spawn(curate_notifications::task(MAIN_CURRENCIES, SERVER_FIREBASE_API_KEY, DATABASE_CONNECTION_STRING));

    router::run(SERVER_REST_API_PORT, &db_pool).await;
}

fn greetings() {
    println!("\n*********************************");
    println!("Curate-server <<{}>> is starting at: {}", ENVIRONMENT, Utc::now().format("%H:%M"));
    println!("*********************************\n");
}