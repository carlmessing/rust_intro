use warp::reply::json;
use warp::http::StatusCode;
use warp::reply::{Json, WithStatus};
use crate::schemas::multiply::MultiplyBodyPOST;

/// This function implements the business logic of the operation *multiplier*.
/// It is called by requests to ```/multiply``` with method ``POST``.
/// All inputs are validated before passed into this function.
pub fn handler(body: MultiplyBodyPOST) -> Result<WithStatus<Json>, warp::Rejection> {
    // business logic
    let result = body.a * body.b;
    Ok(warp::reply::with_status(
        json(&serde_json::json!({"result": result})),
        StatusCode::OK
    ))
}