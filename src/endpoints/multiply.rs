use std::panic;
use crate::endpoints::log_internal_error;
use crate::handlers;
use crate::schemas::component_types::Operands;
use crate::utils::{reply_internal_error, reply_ok};
use crate::handlers::multiplier::ReturnValue;

/// `[POST] /multiply` endpoint,
/// replies `500 Internal Server Error` if handler panicks
pub async fn post(body: Operands) -> Result<impl warp::Reply, warp::Rejection> {
    let result = panic::catch_unwind(|| {handlers::multiplier::handler(body)});
    match result {
        Ok(ReturnValue::ProductOfAAndB(x)) => Ok(reply_ok(&x)),
        Err(err) => { 
            log_internal_error(err);
            Ok(reply_internal_error())
        }
    }
}