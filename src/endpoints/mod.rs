use serde_json::json;
use warp::http::StatusCode;
use warp::reject;
use warp::reject::{InvalidQuery, MethodNotAllowed};

pub(crate) mod add;
pub(crate) mod subtract;
pub(crate) mod multiply;
pub(crate) mod divide;
pub(crate) mod square;

#[derive(Debug)]
pub(crate) struct JsonBodyError {
    pub(crate) message: String,
}

impl reject::Reject for JsonBodyError {}

pub async fn recover(err: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if err.is_not_found() {
        return Ok(warp::reply::with_status(
            warp::reply::json(&json!({ "error": "Not found" })),
            StatusCode::NOT_FOUND,
        ));
    }
    
    if let Some(_) = err.find::<InvalidQuery>() {
        let json = warp::reply::json(&json!({
            "error": "invalid or missing query parameter(s)"
        }));
        return Ok(warp::reply::with_status(json, StatusCode::BAD_REQUEST));
    }
    
    if let Some(e) = err.find::<JsonBodyError>() {
        let json = warp::reply::json(&json!({
            "error": "Invalid request body",
            "details": e.message,
        }));
        return Ok(warp::reply::with_status(json, StatusCode::BAD_REQUEST));
    }

    if let Some(_) = err.find::<MethodNotAllowed>() {
        let json = warp::reply::json(&json!({
            "error": "method not allowed"
        }));
        return Ok(warp::reply::with_status(json, StatusCode::BAD_REQUEST));
    }

    // fallback error
    Ok(warp::reply::with_status(
        warp::reply::json(&json!({
            "error": "Unhandled rejection"
        })),
        StatusCode::INTERNAL_SERVER_ERROR,
    ))
}