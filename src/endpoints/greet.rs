use std::convert::Infallible;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct QueryParams {
    version: Option<String>,
}

#[derive(Serialize)]
struct GreetResponseGET {
    message: String,
}

#[derive(Serialize)]
struct GreetResponseGETerror {
    error: String,
}

// greet GET handler
pub async fn get(params: QueryParams) -> Result<impl warp::Reply, warp::Rejection> {
    if params.version.is_none() {
        return Ok(warp::reply::with_status(
            warp::reply::json(&GreetResponseGETerror {error: "missing argument".to_string()}), 
            warp::http::StatusCode::BAD_REQUEST)
        );
    }
    
    // business logic
    let response = GreetResponseGET {
        message: "Hello someone".to_string(),
    };
    
    Ok(warp::reply::with_status(
        warp::reply::json(&response),
        warp::http::StatusCode::OK
    ))
}

#[derive(Serialize)]
struct GreetResponsePOST {
    message: String,
}

// greet POST handler
pub async fn post(name: String) -> Result<impl warp::Reply, Infallible> {
    let response = GreetResponsePOST {
        message: format!("Hello, {}!", name),
    };
    Ok(warp::reply::with_status(
        warp::reply::json(&response),
        warp::http::StatusCode::OK
    ))
}