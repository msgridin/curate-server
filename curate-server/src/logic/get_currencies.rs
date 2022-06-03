use warp::{reject, Rejection, Reply};
use crate::DBPool;
use crate::data;
use crate::data::models::GetCurrenciesResponse;
use crate::error::ServerError;

pub(crate) async fn invoke(db_pool: DBPool) -> Result<impl Reply, Rejection> {

    println!("get_currencies");
    let currencies = data::db::currencies::read_currencies(&db_pool).await
       .map_err(|e| reject::custom(ServerError::from(e)))?;

    let response = GetCurrenciesResponse {
        currencies
    };
    let json = warp::reply::json(&response);

    Ok(json)
}

