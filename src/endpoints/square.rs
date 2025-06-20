use std::panic;
use crate::handlers;
use crate::utils::reply_internal_error;

/// square GET endpoint
pub async fn get(base: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let result = panic::catch_unwind(|| {handlers::square::get(base)});
    match result {
        Ok(x) => x,
        Err(_) => Ok(reply_internal_error())
    }
}