use chrono::Utc;
use sqlx::{Pool, Postgres};
use curate_import;
use crate::config::{ ENVIRONMENT, SERVER_REST_API_PORT, DATABASE_CONNECTION_STRING };

mod config;
mod data;
mod router;
mod logic;
mod error;

pub(crate) type DBPool = Pool<Postgres>;

#[tokio::main]
async fn main(){
    greetings();

    let db_pool = data::db::init(DATABASE_CONNECTION_STRING).await;

    tokio::spawn(curate_import::task(DATABASE_CONNECTION_STRING));

    router::run(SERVER_REST_API_PORT, &db_pool).await;
}

fn greetings() {
    println!("\n*********************************");
    println!("Curate-server <<{}>> is starting at: {}", ENVIRONMENT, Utc::now().format("%H:%M").to_string());
    println!("*********************************\n");
}