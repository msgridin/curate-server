use warp::{reject, Rejection, Reply};
use crate::DBPool;
use crate::data;
use crate::data::models::{GetRateNotificationSubscriptionsResponse};
use crate::error::ServerError;

pub(crate) async fn invoke(firebase_token: String, db_pool: DBPool) -> Result<impl Reply, Rejection> {

    println!("get_rate_subscriptions");
    let subscriptions = data::db::read_rate_subscriptions(firebase_token.as_str(), &db_pool).await
       .map_err(|e| reject::custom(ServerError::from(e)))?;

    let response = GetRateNotificationSubscriptionsResponse {
        subscriptions
    };
    let json = warp::reply::json(&response);

    Ok(json)
}

