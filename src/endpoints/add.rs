use std::panic;
use serde::Deserialize;
use tracing::{info, error, Instrument, Span};

use crate::endpoints::log_internal_error;
use crate::handlers;
use crate::utils::{reply_internal_error, reply_ok};
use crate::handlers::adder::ReturnValue;

#[derive(Deserialize)]
pub struct GetQueryParams {
    pub(crate) a: i32,
    pub(crate) b: i32
}

/// `[GET] /add` endpoint,
/// replies `500 Internal Server Error` if handler panicks
pub async fn get(params: GetQueryParams) -> Result<impl warp::Reply, warp::Rejection>{
    let span = tracing::span!(
        tracing::Level::INFO,
        "add_handler",
        method = "GET",
        endpoint = "/add",
        a = params.a,
        b = params.b
    );

    async {
        info!("Received request to /add with query params");
        let result = panic::catch_unwind(|| {handlers::adder::handler(params)});
        match result {
            Ok(body @ ReturnValue::SumOfAAndB { .. }) => {
                info!(
                    result = ?body,
                    "Successfully computed sum in /add"
                );
                Ok(reply_ok(&body))
            },
            Err(err) => {
                error!("❌ Panic occurred in /add handler: {:?}", err);
                log_internal_error(err);
                Ok(reply_internal_error())
            }
        }
    }
    .instrument(span) // Attach the span to this async block
    .await
}