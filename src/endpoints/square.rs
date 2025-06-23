use std::panic;
use crate::handlers;
use crate::utils::reply_internal_error;
use tracing::{info_span, error, Instrument};

/// `[GET] /square/{n}` endpoint,
/// replies `500 Internal Server Error` if handler panicks
pub async fn get(base: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let span = info_span!("handle_square", base = base);

    async move {
        let result = panic::catch_unwind(|| {handlers::squarer::handler(base)});
        match result {
            Ok(x) => x,
            Err(_) => {
                error!("handler panicked in /square");
                Ok(reply_internal_error())
            }
        }
    }
    .instrument(span)
    .await
}