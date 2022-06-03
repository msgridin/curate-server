use warp::{reject, Rejection, Reply};
use crate::{data, DBPool};
use crate::data::models::CurrentRate;
use crate::error::ServerError;
use crate::config::MAIN_CURRENCIES;
use crate::logic::util::get_multiplier;

pub(crate) async fn invoke(from_currency_id: String, to_currency_id: String, db_pool: DBPool) -> Result<impl Reply, Rejection> {
    println!("get_current_rate {} {}", from_currency_id, to_currency_id);
    let rate = if MAIN_CURRENCIES.contains(&from_currency_id.as_str()) {
        data::db::rates::read_current_rate(from_currency_id.as_str(), to_currency_id.as_str(), &db_pool).await
            .map_err(|e| reject::custom(ServerError::from(e)))?
    } else {
        let current = data::db::rates::read_current_rate(to_currency_id.as_str(), from_currency_id.as_str(), &db_pool).await
            .map_err(|e| reject::custom(ServerError::from(e)))?;
        if current == 0.0 {
            0.0
        } else {
            1.0 / current
        }
    };

    let multiplier = get_multiplier(rate);

    let response: CurrentRate = CurrentRate {
        rate: rate * (multiplier as f64),
        multiplier,
    };

    println!("{:?}", response);
    let json = warp::reply::json(&response);
    Ok(json)
}


