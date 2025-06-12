use serde::{Deserialize, Serialize};
use crate::handlers;
use crate::handlers::reply_internal_error;

// health GET endpoint
pub async fn get() -> Result<impl warp::Reply, warp::Rejection> {
    match handlers::health::get() {
        Ok(x) => Ok(x),
        Err(_) => Ok(reply_internal_error())
    }
}

#[derive(Serialize, Deserialize)]
pub struct BodyPOST {
    pub name: String,
    pub age: i32
}

// health POST endpoint
pub async fn post(data: BodyPOST) -> Result<impl warp::Reply, warp::Rejection> {
    match handlers::health::post(data) {
        Ok(x) => Ok(x),
        Err(_) => Ok(reply_internal_error())
    }
}