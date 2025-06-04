use serde::Serialize;
use warp::reply::json;
use crate::endpoints::health::BodyPOST;
use warp::http::StatusCode;
use crate::handlers::StatusResponse;

#[derive(Serialize)]
struct HealthResponseGET {
    status: String,
}

// health GET handler
pub fn get() -> StatusResponse {
    // business logic
    let response = HealthResponseGET {
        status: "OK".to_string()+" GET",
    };
    
    StatusResponse {
        body: json(&response), 
        status_code: StatusCode::OK
    }
}

#[derive(Serialize)]
struct HealthResponsePOST {
    name: String,
    age: i32
}

// health POST handler
pub fn post(data: BodyPOST) -> StatusResponse {
    // business logic
    let response = HealthResponsePOST {
        name: data.name,
        age: data.age
    };
    
    StatusResponse {
        body: json(&response),
        status_code: StatusCode::OK
    }
}