use std::error::Error;
use warp::reject::Reject;

#[derive(Debug)]
pub(crate) struct ServerError(pub(crate) String);

impl Reject for ServerError {}

impl From<Box<dyn Error>> for ServerError {
    fn from(e: Box<dyn Error>) -> Self {
        ServerError(format!("{:?}", e))
    }
}

impl From<serde_json::Error> for ServerError {
    fn from(e: serde_json::Error) -> Self {
        ServerError(format!("{:?}", e))
    }
}
