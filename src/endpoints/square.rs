use std::fmt::Debug;
use std::panic;
use crate::{endpoints, handlers};
use crate::handlers::squarer::ReturnValue;
use crate::utils::{reply_internal_error, reply_ok};

/// `[GET] /square/{n}` endpoint,
/// replies `500 Internal Server Error` if handler panicks
pub async fn get(base: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let result = panic::catch_unwind(|| {handlers::squarer::handler(base)});
    match result {
        Ok(ReturnValue::ResultOfNSquared(x)) => Ok(reply_ok(&x)),
        Err(err) => {
            endpoints::log_internal_error(err);
            Ok(reply_internal_error())
        }
    }
}