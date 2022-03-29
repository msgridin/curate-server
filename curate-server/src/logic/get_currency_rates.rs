use warp::{Rejection, Reply};
use crate::DBPool;

pub(crate) async fn invoke(_currency: String, _db_pool: DBPool) -> Result<impl Reply, Rejection> {

    Ok("OK".to_string())
}

