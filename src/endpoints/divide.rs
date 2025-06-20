use std::panic;
use crate::handlers;
use crate::schemas::divide::DivideBodyPOST;
use crate::utils::reply_internal_error;

/// divide POST endpoint
pub async fn post(body: DivideBodyPOST) -> Result<impl warp::Reply, warp::Rejection> {
    let result = panic::catch_unwind(|| {handlers::divide::post(body)});
    match result {
        Ok(x) => x,
        Err(_) => Ok(reply_internal_error())
    }
}