use serde_json::json;
use warp::http::StatusCode;
use warp::reply::{json, Json, WithStatus};

/// square GET handler
pub fn get(base: i32) -> Result<WithStatus<Json>, warp::Rejection> {
    // business logic
    let result = base * base;
    Ok(warp::reply::with_status(
        json(&json!({"result": result})),
        StatusCode::OK
    ))
}