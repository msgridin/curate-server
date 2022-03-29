use warp::{reject, Rejection, Reply};
use crate::DBPool;
use crate::data;
use crate::error::ServerError;


pub(crate) async fn invoke(db_pool: DBPool) -> Result<impl Reply, Rejection> {

    let currencies = data::db::read_currencies(&db_pool).await
        .map_err(|e| reject::custom(ServerError::from(e)))?;
    let json = serde_json::to_string(&currencies)
        .map_err(|e| reject::custom(ServerError::from(e)))?;


    Ok(json)
}

