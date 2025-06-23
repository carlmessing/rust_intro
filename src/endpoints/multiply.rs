use std::panic;
use crate::handlers;
use crate::schemas::multiply::MultiplyBodyPOST;
use crate::utils::reply_internal_error;
use tracing::{info_span, error, Instrument};

/// `[POST] /multiply` endpoint,
/// replies `500 Internal Server Error` if handler panicks
pub async fn post(body: MultiplyBodyPOST) -> Result<impl warp::Reply, warp::Rejection> {
    let span = info_span!("handle_add", a = body.a, b = body.b);

    async move{
        let result = panic::catch_unwind(|| {handlers::multiplier::handler(body)});
        match result {
            Ok(x) => x,
            Err(_) => {
                error!("handler panicked in /multiply");
                Ok(reply_internal_error())
            }
        }
    }
    .instrument(span)
    .await
}