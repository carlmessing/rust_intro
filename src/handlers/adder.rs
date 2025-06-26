use serde_json::json;
use warp::http::StatusCode;
use crate::endpoints::add::GetQueryParams;
use warp::reply::{json, Json, WithStatus};

/// This function implements the business logic of the operation *adder*.
/// It is called by requests to /add with method GET.
/// All inputs are validated before being passed into this function.
pub fn handler(params: GetQueryParams) -> Result<WithStatus<Json>, warp::Rejection> {
    let result = params.a + params.b;
    Ok(warp::reply::with_status(
        json(&json!({"result": result})),
        StatusCode::OK
    ))
}