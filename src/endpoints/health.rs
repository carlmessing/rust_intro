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

/// infoz endpoint for retriving certain informations about the application
pub(crate) async fn infoz(key: String) -> Result<impl warp::Reply, warp::Rejection> {
    let info = match handlers::health::infoz(&key) { 
        None => ("No information available".to_string(), StatusCode::NOT_FOUND),
        x => (x.unwrap(), StatusCode::OK)
    };
    
    Ok(warp::reply::with_status(
        json(&json!({&key: info.0})),
        info.1
    ))
}