use std::convert::Infallible;
use serde::{Deserialize, Serialize};
use crate::handlers;

#[derive(Deserialize)]
pub struct QueryParams {
    pub(crate) version: Option<String>,
}

#[derive(Serialize)]
struct GreetResponseGETerror {
    error: String,
}

// greet GET endpoint
pub async fn get(params: QueryParams) -> Result<impl warp::Reply, warp::Rejection> {
    // validation
    if params.version.is_none() {
        return Ok(warp::reply::with_status(
            warp::reply::json(&GreetResponseGETerror {error: "missing argument".to_string()}), 
            warp::http::StatusCode::BAD_REQUEST)
        );
    }
    
    let response = handlers::greet::get(params);
    Ok(warp::reply::with_status(response.body, response.status_code))
}


// greet POST endpoint
pub async fn post(name: String) -> Result<impl warp::Reply, Infallible> {
    let response = handlers::greet::post(name);
    Ok(warp::reply::with_status(response.body, response.status_code))
}