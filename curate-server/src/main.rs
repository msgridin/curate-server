use chrono::Utc;
use sqlx::{Pool, Postgres};
use curate_import;
use crate::config::{ ENVIRONMENT, SERVER_REST_API_PORT, DATABASE_CONNECTION_STRING };

mod config;
mod data;
mod service;
mod logic;

pub(crate) type DBPool = Pool<Postgres>;

#[tokio::main]
async fn main(){
    greetings();

    let db_pool = data::db::create_pool(DATABASE_CONNECTION_STRING).await.expect("ERROR: CREATE DATABASE POOL");
    data::db::init_db(&db_pool).await.expect("ERROR: INIT DATABASE");

    tokio::spawn(curate_import::task(DATABASE_CONNECTION_STRING));

    service::api::run(SERVER_REST_API_PORT, &db_pool).await;
}

fn greetings() {
    println!("\n*********************************");
    println!("Curate-server <<{}>> is starting at: {}", ENVIRONMENT, Utc::now().format("%H:%M").to_string());
    println!("*********************************\n");
}