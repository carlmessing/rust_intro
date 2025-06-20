use crate::handlers;
use crate::schemas::multiply::MultiplyBodyPOST;
use crate::utils::reply_internal_error;

/// multiply POST endpoint
pub async fn post(body: MultiplyBodyPOST) -> Result<impl warp::Reply, warp::Rejection> {
    match handlers::multiply::post(body) {
        Ok(x) => Ok(x),
        Err(_) => Ok(reply_internal_error())
    }
}