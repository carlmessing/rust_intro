use std::panic;
use warp::http::StatusCode;
use warp::reply::json;
use crate::{handlers};
use crate::schemas::component_types::{Operands, Result};
use crate::utils::{reply_internal_error, reply_ok};

/// `[POST] /divide` endpoint,
/// replies `500 Internal Server Error` if handler panicks
pub async fn post(body: Operands) -> core::result::Result<impl warp::Reply, warp::Rejection> {
    let result = panic::catch_unwind(|| {handlers::divider::handler(body)});
    match result {
        Ok(Ok(x)) => Ok(reply_ok(&Result { result: x })),
        Ok(Err(e)) => Ok(warp::reply::with_status(
            json(&e),
            StatusCode::BAD_REQUEST
        )),
        _ => Ok(reply_internal_error())
    }
}