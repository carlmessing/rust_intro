use std::panic;
use crate::handlers;
use crate::utils::reply_internal_error;

/// `[GET] /square/{n}` endpoint,
/// replies `500 Internal Server Error` if handler panicks
pub async fn get(base: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let result = panic::catch_unwind(|| {handlers::squarer::handler(base)});
    match result {
        Ok(x) => x,
        Err(_) => Ok(reply_internal_error())
    }
}