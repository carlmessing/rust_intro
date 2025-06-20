use serde_json::json;
use warp::http::StatusCode;
use crate::endpoints::add::GetQueryParams;
use warp::reply::{json, Json, WithStatus};

/// add GET handler
pub fn get(params: GetQueryParams) -> Result<WithStatus<Json>, warp::Rejection> {
    let result = params.a + params.b;
    Ok(warp::reply::with_status(
        json(&json!({"result": result})),
        StatusCode::OK
    ))
}