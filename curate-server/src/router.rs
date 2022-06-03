use std::convert::Infallible;
use warp::{Filter, http, Rejection, Reply};
use warp::http::StatusCode;
use crate::{DBPool, logic};

pub(crate) async fn run(port: u16, db_pool: &DBPool) {
    warp::serve(get_routes(db_pool))
        .run(([0, 0, 0, 0], port)).await;
}

pub(crate) fn get_routes(
    db_pool: &DBPool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let get_currencies = warp::get()
        .and(warp::path("get_currencies"))
        .and(with_db(db_pool.clone()))
        .and_then(logic::get_currencies::invoke);
    let get_currency_rates = warp::get()
        .and(warp::path("get_currency_rates"))
        .and(warp::header::<String>("currency"))
        .and(warp::header::<String>("foreign_currency"))
        .and(warp::header::<i64>("period"))
        .and(with_db(db_pool.clone()))
        .and_then(logic::get_currency_rates::invoke);
    let get_current_rate = warp::get()
        .and(warp::path("get_current_rate"))
        .and(warp::header::<String>("from_currency"))
        .and(warp::header::<String>("to_currency"))
        .and(with_db(db_pool.clone()))
        .and_then(logic::get_current_rate::invoke);
    let get_rate_subscriptions = warp::get()
        .and(warp::path("get_rate_subscriptions"))
        .and(warp::header::<String>("firebase_token"))
        .and(with_db(db_pool.clone()))
        .and_then(logic::get_rate_subscriptions::invoke);
    let subscribe_rate_notification = warp::get()
        .and(warp::path("subscribe_rate_notification"))
        .and(warp::header::<String>("from_currency"))
        .and(warp::header::<String>("to_currency"))
        .and(warp::header::<String>("firebase_token"))
        .and(with_db(db_pool.clone()))
        .and_then(logic::subscribe_rate_notification::invoke);
    let unsubscribe_rate_notification = warp::get()
        .and(warp::path("unsubscribe_rate_notification"))
        .and(warp::header::<String>("from_currency"))
        .and(warp::header::<String>("to_currency"))
        .and(warp::header::<String>("firebase_token"))
        .and(with_db(db_pool.clone()))
        .and_then(logic::unsubscribe_rate_notification::invoke);

    // CORS
    let cors = warp::cors()
        .allow_methods(&[http::Method::GET])
        .allow_headers(vec![
            http::header::CONTENT_TYPE,
            http::header::AUTHORIZATION,
        ])
        .allow_any_origin();

    get_currencies
        .or(get_currency_rates)
        .or(get_current_rate)
        .or(get_rate_subscriptions)
        .or(subscribe_rate_notification)
        .or(unsubscribe_rate_notification)
        .recover(handle_rejection)
        .with(cors)
}

pub(crate) fn with_db(db_pool: DBPool) -> impl Filter<Extract = (DBPool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let error: String;
    let code;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        error = "Not found".to_string();
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        code = StatusCode::BAD_REQUEST;
        error = format!("Invalid body: {:?}", e);
    } else if let Some(e) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        error = format!("Method not allowed: {:?}", e);
    } else {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        error = format!("Internal server error: {:?}", err);
    }

    println!("{}", error);

    Ok(warp::reply::with_status(error, code))
}
