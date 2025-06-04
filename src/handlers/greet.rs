use serde::Serialize;
use warp::http::StatusCode;
use warp::reply::json;
use crate::endpoints::greet::QueryParams;
use crate::handlers::StatusResponse;

#[derive(Serialize)]
struct GreetResponseGET {
    message: String,
}

// greet GET handler
pub fn get(params: QueryParams) -> StatusResponse {
    // business logic
    let response = GreetResponseGET {
        message: params.version.unwrap(),
    };
    
    StatusResponse {
        body: json(&response),
        status_code: StatusCode::OK
    }
}

#[derive(Serialize)]
struct GreetResponsePOST {
    message: String,
}

// greet POST handler
pub fn post(name: String) -> StatusResponse {
    // business logic
    let response = GreetResponsePOST {
        message: format!("Hello, {}!", name),
    };
    
    StatusResponse {
        body: json(&response),
        status_code: StatusCode::OK
    }
}
