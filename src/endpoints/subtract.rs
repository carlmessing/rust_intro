use std::panic;
use serde::Deserialize;
use crate::handlers;
use crate::utils::reply_internal_error;

#[derive(Deserialize)]
pub struct GetQueryParams {
    pub(crate) a: i32,
    pub(crate) b: i32
}

/// `[GET] /subtract` endpoint,
/// replies `500 Internal Server Error` if handler panicks
pub async fn get(params: GetQueryParams) -> Result<impl warp::Reply, warp::Rejection>{
    let result = panic::catch_unwind(|| {handlers::subtractor::handler(params)});
    match result {
        Ok(x) => x,
        Err(_) => Ok(reply_internal_error())
    }
}