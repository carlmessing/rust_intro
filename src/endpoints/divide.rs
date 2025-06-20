use std::panic;
use crate::handlers;
use crate::schemas::divide::DivideBodyPOST;
use crate::utils::reply_internal_error;

/// `[POST] /divide` endpoint,
/// replies `500 Internal Server Error` if handler panicks
pub async fn post(body: DivideBodyPOST) -> Result<impl warp::Reply, warp::Rejection> {
    let result = panic::catch_unwind(|| {handlers::divider::handler(body)});
    match result {
        Ok(x) => x,
        Err(_) => Ok(reply_internal_error())
    }
}