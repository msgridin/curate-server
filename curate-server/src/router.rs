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
        .and(with_db(db_pool.clone()))
        .and_then(logic::get_currency_rates::invoke);

    // CORS
    let cors = warp::cors()
        .allow_methods(&[http::Method::GET])
        .allow_headers(vec![
            http::header::CONTENT_TYPE,
            http::header::AUTHORIZATION,
        ])
        .allow_any_origin();

    let routes = get_currencies
        .or(get_currency_rates)
        .recover(handle_rejection)
        .with(cors);

    routes
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

    Ok(warp::reply::with_status(error, code))
}
