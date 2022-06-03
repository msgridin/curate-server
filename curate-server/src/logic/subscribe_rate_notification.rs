use warp::{reject, Rejection, Reply};
use crate::DBPool;
use crate::data;
use crate::error::ServerError;

pub(crate) async fn invoke(from_currency: String, to_currency: String, firebase_token: String, db_pool: DBPool) -> Result<impl Reply, Rejection> {

    println!("subscribe_rate_notification");
    let response = data::db::subscriptions::subscribe_rate_notification(from_currency.as_str(), to_currency.as_str(), firebase_token.as_str(), &db_pool).await
       .map_err(|e| reject::custom(ServerError::from(e)))?;

    let json = warp::reply::json(&response);

    Ok(json)
}

