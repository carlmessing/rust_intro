use serde::Serialize;
use warp::reply::{Json, WithStatus};
use crate::endpoints::greet;
use crate::utils::reply_ok;

#[derive(Serialize)]
struct GreetResponseGET {
    message: i32,
}

// greet GET handler
pub fn get(params: greet::GetQueryParams) -> Result<WithStatus<Json>, warp::Rejection> {
    // business logic
    let response = GreetResponseGET {
        message: params.version,
    };

    Ok(reply_ok(&response))
}

#[derive(Serialize)]
struct GreetResponsePOST {
    message: String,
}

// greet POST handler
pub fn post(name: String, body: serde_json::Value) -> Result<WithStatus<Json>, warp::Rejection> {
    // business logic
    let response = GreetResponsePOST {
        message: format!("Hello, {} aka {}!", name, body.get("description").unwrap()),
    };

    Ok(reply_ok(&response))
}
