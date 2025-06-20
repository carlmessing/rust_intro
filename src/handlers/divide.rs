use warp::reply::json;
use warp::http::StatusCode;
use warp::reply::{Json, WithStatus};
use crate::schemas::divide::DivideBodyPOST;

/// divide POST handler
pub fn post(body: DivideBodyPOST) -> Result<WithStatus<Json>, warp::Rejection> {
    // business logic
    if body.b == 0 {
        return Ok(warp::reply::with_status(
            json(&serde_json::json!({"error": "b can not be zero"})),
            StatusCode::BAD_REQUEST
        ))
    }
    let result = body.a / body.b;
    Ok(warp::reply::with_status(
        json(&serde_json::json!({"result": result})),
        StatusCode::OK
    ))
}