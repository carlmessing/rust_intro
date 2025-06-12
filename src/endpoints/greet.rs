use std::convert::Infallible;
use serde::{Deserialize, Serialize};
use warp::hyper::body::Bytes;
use warp::reject::InvalidQuery;
use crate::handlers;
use crate::handlers::{reply_invalid_parameters, reply_notfound};

#[derive(Deserialize)]
pub struct GetQueryParams {
    pub(crate) version: Option<i32>,
}

#[derive(Serialize)]
struct GreetResponseGETerror {
    error: String,
}

// greet GET endpoint
pub async fn get(params: GetQueryParams) -> Result<impl warp::Reply, warp::Rejection> {
    // validation
    if params.version.is_none() {
        return Ok(warp::reply::with_status(
            warp::reply::json(&GreetResponseGETerror {error: "missing argument".to_string()}), 
            warp::http::StatusCode::BAD_REQUEST)
        );
    }
    
    Ok(handlers::greet::get(params))
}

pub async fn recover_get(err: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(_) = err.find::<InvalidQuery>() { return Ok(reply_invalid_parameters()) }
    Ok(reply_notfound())
}

// greet POST endpoint
pub async fn post(name: String, body: Bytes) -> Result<impl warp::Reply, Infallible> {
    // body validation
    let body = match serde_json::from_slice::<serde_json::Value>(&body) {
        Ok(x) => x,
        Err(_) => {
            return Ok(warp::reply::with_status(
                warp::reply::json(&GreetResponseGETerror {error: "Invalid JSON in Body".to_string()}),
                warp::http::StatusCode::BAD_REQUEST)
            );
        }
    };
    if let Err(e) = crate::utils::validator::validate_message_schema(
        std::path::Path::new("./src/schemas/greet_id_post_payload.json"),
        &body,
    ) {
        println!(
            "Failed to validate message schema: weather_alert\nOriginal message: {:#?}\nError: {}",
            &body, e
        );
        return Ok(warp::reply::with_status(
            warp::reply::json(&GreetResponseGETerror {error: "Invalid Body".to_string()}),
            warp::http::StatusCode::BAD_REQUEST)
        );
    }

    Ok(handlers::greet::post(name, body))
}