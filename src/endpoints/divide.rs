use std::panic;
use warp::http::StatusCode;
use warp::reply::json;
use crate::{handlers};
use crate::handlers::divider::ReturnValue;
use crate::schemas::component_types::{Operands};
use crate::utils::{reply_internal_error, reply_ok};

/// `[POST] /divide` endpoint,
/// replies `500 Internal Server Error` if handler panicks
pub async fn post(body: Operands) -> Result<impl warp::Reply, warp::Rejection> {
    let result = panic::catch_unwind(|| {handlers::divider::handler(body)});
    match result {
        Ok(ReturnValue::ResultOfAB(x)) => Ok(reply_ok(&x)),
        Ok(ReturnValue::DivisionByZeroError(x)) => Ok(warp::reply::with_status(
            json(&x),
            StatusCode::BAD_REQUEST
        )),
        _ => Ok(reply_internal_error())
    }
}