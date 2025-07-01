use serde_json::json;
use warp::http::StatusCode;
use warp::reject;
use warp::reject::{InvalidQuery, MethodNotAllowed};
use warp::reply::json;
use std::any::Any;
use log::error;
use crate::utils::{reply_forbidden_method, reply_notfound};

pub(crate) mod add;
pub(crate) mod subtract;
pub(crate) mod multiply;
pub(crate) mod divide;
pub(crate) mod square;
pub(crate) mod health;

#[derive(Debug)]
pub(crate) struct BodyInputError {
    pub(crate) message: String,
}

impl reject::Reject for BodyInputError {}

#[derive(Debug)]
pub(crate) struct QueryInputError {
    pub(crate) message: String,
}

impl reject::Reject for QueryInputError {}

pub async fn recover(err: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if err.is_not_found() {
        return Ok(reply_notfound());
    }
    
    if let Some(e) = err.find::<QueryInputError>() {
        let json = warp::reply::json(&json!({
            "error": "Invalid parameter(s)",
            "details": e.message,
        }));
        return Ok(warp::reply::with_status(json, StatusCode::BAD_REQUEST));
    }
    
    if let Some(e) = err.find::<BodyInputError>() {
        let json = warp::reply::json(&json!({
            "error": "Invalid request body",
            "details": e.message,
        }));
        return Ok(warp::reply::with_status(json, StatusCode::BAD_REQUEST));
    }

    if let Some(_) = err.find::<InvalidQuery>() {
        return Ok(warp::reply::with_status(
            json(&json!({"error": "missing parameter(s)"})),
            StatusCode::BAD_REQUEST
        ))
    }

    if let Some(_) = err.find::<MethodNotAllowed>() {
        return Ok(reply_forbidden_method());
    }

    // fallback error
    Ok(warp::reply::with_status(
        warp::reply::json(&json!({
            "error": "Unhandled rejection"
        })),
        StatusCode::INTERNAL_SERVER_ERROR,
    ))
}

pub fn log_internal_error(err: Box<dyn Any + Send>) {
    let message = "and unhandled internal error occured";
    if let Some(s) = err.downcast_ref::<&'static str>() {
        error!("{}: {}", message, s);
    } else if let Some(s) = err.downcast_ref::<String>() {
        error!("{}: {}", message, s);
    } else {
        error!("{}", message);
    }
}