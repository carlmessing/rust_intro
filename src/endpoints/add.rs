use serde::Deserialize;
use crate::handlers;
use crate::utils::reply_internal_error;

#[derive(Deserialize)]
pub struct GetQueryParams {
    pub(crate) a: i32,
    pub(crate) b: i32
}

/// add GET endpoint
pub async fn get(params: GetQueryParams) -> Result<impl warp::Reply, warp::Rejection>{
    match handlers::add::get(params) {
        Ok(x) => Ok(x),
        Err(_) => Ok(reply_internal_error())
    }
}