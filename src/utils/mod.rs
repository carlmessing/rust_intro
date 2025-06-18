use warp::reply::{json, Json, WithStatus};
use warp::http::StatusCode;
use serde::Serialize;
use log::error;

pub mod validator;

pub fn reply_ok<T: Serialize>(response: &T) -> WithStatus<Json> {
    warp::reply::with_status(
        json(response),
        StatusCode::OK
    )
}

pub fn reply_invalid_parameters() -> WithStatus<Json> {
    error!("invalid parameter(s)");
    warp::reply::with_status(
        json(&serde_json::json!({"error": "invalid parameter(s)"})),
        StatusCode::BAD_REQUEST
    )
}

pub fn reply_invalid_body() -> WithStatus<Json> {
    error!("invalid body");
    warp::reply::with_status(
        json(&serde_json::json!({"error": "invalid body"})),
        StatusCode::BAD_REQUEST)

}

pub fn reply_notfound() -> WithStatus<Json> {
    warp::reply::with_status(
        json(&serde_json::json!({"error": "not found"})),
        StatusCode::NOT_FOUND
    )
}

pub fn reply_internal_error() -> WithStatus<Json> {
    error!("internal error");
    warp::reply::with_status(
        json(&serde_json::json!({"error": "an internal error occured"})),
        StatusCode::INTERNAL_SERVER_ERROR
    )
}