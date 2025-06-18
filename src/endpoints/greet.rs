use std::convert::Infallible;
use serde::Deserialize;
use warp::hyper::body::Bytes;
use crate::handlers;
use crate::utils::{reply_internal_error, reply_invalid_body};

#[derive(Deserialize)]
pub struct GetQueryParams {
    pub(crate) version: i32,
}

// greet GET endpoint
pub async fn get(params: GetQueryParams) -> Result<impl warp::Reply, warp::Rejection> {
    match handlers::greet::get(params) {
        Ok(x) => Ok(x),
        Err(_) => Ok(reply_internal_error())
    }
}

// greet POST endpoint
pub async fn post(name: String, body: Bytes) -> Result<impl warp::Reply, Infallible> {
    // body validation
    let body = match serde_json::from_slice::<serde_json::Value>(&body) {
        Ok(x) => x,
        Err(_) => {
            return Ok(reply_invalid_body());
        }
    };
    if let Err(_) = crate::utils::validator::validate_message_schema(
        std::path::Path::new("./src/schemas/greet_id_post_payload.json"),
        &body,
    ) {
        // println!(
        //     "Failed to validate message schema: weather_alert\nOriginal message: {:#?}\nError: {}",
        //     &body, e
        // );
        return Ok(reply_invalid_body());
    }

    match handlers::greet::post(name, body) {
        Ok(x) => Ok(x),
        Err(_) => Ok(reply_internal_error())
    }
}