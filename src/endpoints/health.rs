use serde::{Deserialize, Serialize};
use crate::handlers;

// health GET endpoint
pub async fn get() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(handlers::health::get())
}

#[derive(Serialize, Deserialize)]
pub struct BodyPOST {
    pub name: String,
    pub age: i32
}

// health POST endpoint
pub async fn post(data: BodyPOST) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(handlers::health::post(data))
}