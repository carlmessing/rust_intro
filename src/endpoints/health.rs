use std::convert::Infallible;
use serde::{Deserialize, Serialize};
use crate::handlers;

// health GET endpoint
pub async fn get() -> Result<impl warp::Reply, Infallible> {
    let response = handlers::health::get();
    Ok(warp::reply::with_status(response.body, response.status_code))
}

#[derive(Serialize, Deserialize)]
pub struct BodyPOST {
    pub name: String,
    pub age: i32
}

// health POST endpoint
pub async fn post(data: BodyPOST) -> Result<impl warp::Reply, Infallible> {
    let response = handlers::health::post(data);
    Ok(warp::reply::with_status(response.body, response.status_code))
}