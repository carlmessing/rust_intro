use warp::http::StatusCode;
use warp::reply::json;
use crate::handlers;

/// kubernetes (health check) readyz endpoint;
/// replies with `200` for ready and `500` for not ready
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
/// replies with `200` for ready and `500` for not ready
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

/// infoz endpoint for retrieving certain json-information about the application
pub(crate) async fn infoz() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::with_status(
        json(&handlers::health::infoz()),
        StatusCode::OK
    ))
}