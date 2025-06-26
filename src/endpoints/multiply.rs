use std::panic;
use crate::{handlers, schemas};
use crate::schemas::component_types::Operands;
use crate::utils::{reply_internal_error, reply_ok};
use schemas::component_types::Result;

/// `[POST] /multiply` endpoint,
/// replies `500 Internal Server Error` if handler panicks
pub async fn post(body: Operands) -> core::result::Result<impl warp::Reply, warp::Rejection> {
    let result = panic::catch_unwind(|| {handlers::multiplier::handler(body)});
    match result {
        Ok(Ok(x)) => Ok(reply_ok(&Result { result: x })),
        _ => Ok(reply_internal_error())
    }
}