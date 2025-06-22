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

/// `[GET] /add` endpoint,
/// replies `500 Internal Server Error` if handler panicks
pub async fn get(params: GetQueryParams) -> Result<impl warp::Reply, warp::Rejection>{
    let span = info_span!("handle_add", a = params.a, b = params.b);
    
    async move {
        let result = panic::catch_unwind(|| {handlers::adder::handler(params)});
        match result {
            Ok(x) => x,
            Err(_) => {
                error!("handler panicked in /add");
                Ok(reply_internal_error())
            }
        }
    }
    .instrument(span)
    .await
}