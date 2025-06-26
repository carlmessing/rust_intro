use std::panic;
use serde::{Deserialize};
use crate::{handlers, schemas};
use crate::utils::{reply_internal_error, reply_ok};
use schemas::component_types::Result;

#[derive(Deserialize)]
pub struct GetQueryParams {
    pub(crate) a: i32,
    pub(crate) b: i32
}

/// `[GET] /multiply` endpoint,
/// replies `500 Internal Server Error` if handler panicks
pub async fn get(params: GetQueryParams) -> core::result::Result<impl warp::Reply, warp::Rejection>{
    let result = panic::catch_unwind(|| {handlers::adder::handler(params)});
    match result {
        Ok(Ok(x)) => Ok(reply_ok(&Result { result: x })),
        _ => Ok(reply_internal_error())
    }
}