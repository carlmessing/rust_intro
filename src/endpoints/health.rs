use serde_json::json;
use warp::http::StatusCode;
use warp::reply::json;
use crate::handlers;

/// kubernetes (health check) readyz endpoint;
/// replies with 200 for ready and 500 for not ready
pub(crate) async fn readyz() -> Result<impl warp::Reply, warp::Rejection> {
    if handlers::health::readyz() {
        Ok(warp::reply::with_status(
            "Application is ready",
            StatusCode::OK
        ))
    } else {
        Ok(warp::reply::with_status(
            "Application is not ready",
            StatusCode::INTERNAL_SERVER_ERROR
        ))
    }
}

/// kubernetes (health check) livez endpoint;
/// replies with 200 for ready and 500 for not ready
pub(crate) async fn livez() -> Result<impl warp::Reply, warp::Rejection> {
    if handlers::health::livez() {
        Ok(warp::reply::with_status(
            "Application is alive",
            StatusCode::OK
        ))
    } else {
        Ok(warp::reply::with_status(
            "Application is not alive",
            StatusCode::INTERNAL_SERVER_ERROR
        ))
    }
}

/// infoz endpoint for retrieving certain information about the application
pub(crate) async fn infoz() -> Result<impl warp::Reply, warp::Rejection> {
    let info = json!({
        "title": "Calculator API",
        "version": "1.0.0",
        "description": "A simple calculator with basic arithmetic operations.",
    });
    
    Ok(warp::reply::with_status(
        json(&info),
        StatusCode::OK
    ))
}