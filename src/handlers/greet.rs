use serde::Serialize;
use warp::reply::{Json, WithStatus};
use crate::endpoints::greet;
use crate::handlers::{reply_ok};

#[derive(Serialize)]
struct GreetResponseGET {
    message: i32,
}

pub fn get(params: greet::GetQueryParams) -> WithStatus<Json> {
    // business logic
    let response = GreetResponseGET {
        message: params.version.unwrap(),
    };

    reply_ok(&response)
}

#[derive(Serialize)]
struct GreetResponsePOST {
    message: String,
}

// greet POST handler
pub fn post(name: String, body: serde_json::Value) -> WithStatus<Json> {
    // business logic
    let response = GreetResponsePOST {
        message: format!("Hello, {} aka {}!", name, body.get("description").unwrap()),
    };

    reply_ok(&response)
}
