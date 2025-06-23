use std::panic;
use serde::Deserialize;
use crate::handlers;
use crate::utils::reply_internal_error;
use tracing::{info_span, error, Instrument};

#[derive(Deserialize)]
pub struct GetQueryParams {
    pub(crate) a: i32,
    pub(crate) b: i32
}

/// `[GET] /subtract` endpoint,
/// replies `500 Internal Server Error` if handler panicks
pub async fn get(params: GetQueryParams) -> Result<impl warp::Reply, warp::Rejection>{
    let span = info_span!("handle_subtract", a = params.a, b = params.b);

    async move {
        let result = panic::catch_unwind(|| {handlers::subtractor::handler(params)});
        match result {
            Ok(x) => x,
            Err(_) => {
                error!("handler panicked in /subtract");
                Ok(reply_internal_error())
            }
        }
    }
    .instrument(span)
    .await
}