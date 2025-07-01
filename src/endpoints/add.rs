use std::panic;
use serde::Deserialize;
use crate::handlers;
use crate::utils::{reply_internal_error, reply_ok};
use crate::handlers::adder::ReturnValue;

#[derive(Deserialize)]
pub struct GetQueryParams {
    pub(crate) a: i32,
    pub(crate) b: i32
}

/// `[GET] /multiply` endpoint,
/// replies `500 Internal Server Error` if handler panicks
pub async fn get(params: GetQueryParams) -> Result<impl warp::Reply, warp::Rejection>{
    let result = panic::catch_unwind(|| {handlers::adder::handler(params)});
    match result {
        Ok(body @ ReturnValue::SumOfAAndB {..}) => Ok(reply_ok(&body)),
        _ => Ok(reply_internal_error())
    }
}