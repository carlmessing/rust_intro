use serde_json::json;
use warp::http::StatusCode;
use warp::reply::{json, Json, WithStatus};
use tracing::info;

/// This function implements the business logic of the operation *squarer*.
/// It is called by requests to ```/square/{n}``` with method ``GET``.
/// All inputs are validated before passed into this function.
pub fn handler(base: i32) -> Result<WithStatus<Json>, warp::Rejection> {
    // business logic
    let result = base * base;
    info!(base = base, result, "Executed squarer logic");

    Ok(warp::reply::with_status(
        json(&json!({"result": result})),
        StatusCode::OK
    ))
}