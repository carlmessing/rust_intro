use warp::http::StatusCode;
use warp::reply::Json;

pub mod greet;
pub mod health;

pub struct StatusResponse {
    pub body: Json,
    pub status_code: StatusCode
}