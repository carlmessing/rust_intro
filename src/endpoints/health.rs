use std::convert::Infallible;
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponseGET {
    status: String,
}

// health GET handler
pub async fn get() -> Result<impl warp::Reply, Infallible> {
    let response = HealthResponseGET {
        status: "OK".to_string()+" GET",
    };
    
    Ok(warp::reply::with_status(
        warp::reply::json(&response),
        warp::http::StatusCode::OK
    ))
}

#[derive(Serialize)]
struct HealthResponsePOST {
    result: String,
}

// health POST handler
pub async fn post() -> Result<impl warp::Reply, Infallible> {
    let response = HealthResponsePOST {
        result: "OK".to_string()+" POST",
    };

    Ok(warp::reply::with_status(
        warp::reply::json(&response),
        warp::http::StatusCode::OK
    ))
}