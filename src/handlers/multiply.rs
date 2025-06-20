use warp::reply::json;
use warp::http::StatusCode;
use warp::reply::{Json, WithStatus};
use crate::schemas::multiply::MultiplyBodyPOST;

/// multiply POST handler
pub fn post(body: MultiplyBodyPOST) -> Result<WithStatus<Json>, warp::Rejection> {
    // business logic
    let result = body.a * body.b;
    Ok(warp::reply::with_status(
        json(&serde_json::json!({"result": result})),
        StatusCode::OK
    ))
}