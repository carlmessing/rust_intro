use std::convert::Infallible;
use serde::{Deserialize, Serialize};

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
    name: String,
    age: i32
}

#[derive(Serialize, Deserialize)]
pub struct BodyPOST {
    name: String,
    age: i32
}

// health POST handler
pub async fn post(data: BodyPOST) -> Result<impl warp::Reply, Infallible> {
    let response = HealthResponsePOST {
        name: data.name,
        age: data.age
    };

    Ok(warp::reply::with_status(
        warp::reply::json(&response),
        warp::http::StatusCode::OK
    ))
}