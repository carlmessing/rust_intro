use crate::schemas::component_types::Result;
use std::panic;
use serde::Deserialize;
use crate::handlers;
use crate::utils::{reply_internal_error, reply_ok};

#[derive(Deserialize)]
pub struct GetQueryParams {
    pub(crate) a: i32,
    pub(crate) b: i32
}

/// `[GET] /subtract` endpoint,
/// replies `500 Internal Server Error` if handler panicks
pub async fn get(params: GetQueryParams) -> core::result::Result<impl warp::Reply, warp::Rejection>{
    let result = panic::catch_unwind(|| {handlers::subtractor::handler(params)});
    match result {
        Ok(Ok(x)) => Ok(reply_ok(&Result { result: x })),
        _ => Ok(reply_internal_error())
    }
}