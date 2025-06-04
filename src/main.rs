use warp::Filter;
use serde::{Serialize};
use std::convert::Infallible;
use crate::endpoints::greet::QueryParams;

mod endpoints;
mod handlers;

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

async fn handle_notfound() -> Result<impl warp::Reply, Infallible> {
    let response = ErrorResponse {
        error: "Not found".to_string()
    };
    Ok(warp::reply::with_status(
        warp::reply::json(&response),
        warp::http::StatusCode::NOT_FOUND
    ))
}

#[tokio::main]
async fn main() {
    // health endpoint
    let health_get = warp::get()
        .and(warp::path!("health"))
        .and_then(endpoints::health::get);
    let health_post = warp::post()
        .and(warp::path!("health"))
        .and(warp::body::json())
        .and_then(endpoints::health::post);
    
    // greet endpoint
    let greet_post = warp::post()
        .and(warp::path!("greet" / String))
        .and_then(endpoints::greet::post);
    let greet_get = warp::get()
        .and(warp::path!("greet"))
        .and(warp::path::end())
        .and(warp::query::<QueryParams>())
        .and_then(endpoints::greet::get);
    
    // 404 endpoint
    let notfound = warp::any().and_then(handle_notfound);
    
    // Define routes
    let routes = health_get
        .or(health_post)
        .or(greet_get)
        .or(greet_post)
        .or(notfound);

    // Start server
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
