use std::panic;
use crate::handlers;
use crate::utils::{reply_internal_error, reply_ok};
use crate::schemas::component_types::Result;

/// `[GET] /square/{n}` endpoint,
/// replies `500 Internal Server Error` if handler panicks
pub async fn get(base: i32) -> core::result::Result<impl warp::Reply, warp::Rejection> {
    let result = panic::catch_unwind(|| {handlers::squarer::handler(base)});
    match result {
        Ok(Ok(x)) => Ok(reply_ok(&Result { result: x })),
        _ => Ok(reply_internal_error())
    }
}