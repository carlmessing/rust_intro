use std::panic;
use crate::handlers;
use crate::schemas::divide::DivideBodyPOST;
use crate::utils::reply_internal_error;
use tracing::{info_span, error, Instrument};

/// `[POST] /divide` endpoint,
/// replies `500 Internal Server Error` if handler panicks
pub async fn post(body: DivideBodyPOST) -> Result<impl warp::Reply, warp::Rejection> {
    let span = info_span!("handle_divide", a = body.a, b = body.b);

    async move {
        let result = panic::catch_unwind(|| {handlers::divider::handler(body)});
        match result {
            Ok(x) => x,
            Err(_) =>  {
                error!("handler panicked in /divide");
                Ok(reply_internal_error())
            }
        }
    }
    .instrument(span)
    .await
}