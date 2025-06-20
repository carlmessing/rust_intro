use std::panic;
use crate::handlers;
use crate::schemas::multiply::MultiplyBodyPOST;
use crate::utils::reply_internal_error;

/// `[POST] /multiply` endpoint,
/// replies `500 Internal Server Error` if handler panicks
pub async fn post(body: MultiplyBodyPOST) -> Result<impl warp::Reply, warp::Rejection> {
    let result = panic::catch_unwind(|| {handlers::multiplier::handler(body)});
    match result {
        Ok(x) => x,
        Err(_) => Ok(reply_internal_error())
    }
}