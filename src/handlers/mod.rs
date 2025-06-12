use serde::Serialize;
use warp::http::StatusCode;
use warp::reply::{json, Json, WithStatus};

pub mod greet;
pub mod health;

pub fn reply_ok<T: Serialize>(response: &T) -> WithStatus<Json> {
    warp::reply::with_status(
        json(response),
        StatusCode::OK
    )
}

pub fn reply_invalid_parameters() -> WithStatus<&'static str> {
    warp::reply::with_status(
        "invalid parameters",
        StatusCode::BAD_REQUEST
    )
}

pub fn reply_notfound() -> WithStatus<&'static str> {
    warp::reply::with_status(
        "not found",
        StatusCode::NOT_FOUND
    )
}