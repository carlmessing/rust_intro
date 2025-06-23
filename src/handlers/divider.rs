use warp::reply::json;
use warp::http::StatusCode;
use warp::reply::{Json, WithStatus};
use crate::schemas::divide::DivideBodyPOST;
use tracing::info;

/// This function implements the business logic of the operation *divider*.
/// It is called by requests to /divide with method POST.
/// All inputs are validated before being passed into this function.
pub fn handler(body: DivideBodyPOST) -> Result<WithStatus<Json>, warp::Rejection> {
    // business logic
    if body.b == 0 {
        info!(a = body.a, b = body.b, "User tried dividing by 0");
        return Ok(warp::reply::with_status(
            json(&serde_json::json!({"error": "b can not be zero"})),
            StatusCode::BAD_REQUEST
        ))
    }
    let result = body.a / body.b;
    info!(a = body.a, b = body.b, result, "Executed divider logic");
    Ok(warp::reply::with_status(
        json(&serde_json::json!({"result": result})),
        StatusCode::OK
    ))
}