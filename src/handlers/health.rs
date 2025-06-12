use serde::Serialize;
use warp::reply::{Json, WithStatus};
use crate::endpoints::health::BodyPOST;
use crate::handlers::{reply_ok};

#[derive(Serialize)]
struct HealthResponseGET {
    status: String,
}

// health GET handler
pub fn get() -> WithStatus<Json> {
    // business logic
    let response = HealthResponseGET {
        status: "OK GET".to_string(),
    };
    
    reply_ok(&response)
}

#[derive(Serialize)]
struct HealthResponsePOST {
    name: String,
    age: i32
}

// health POST handler
pub fn post(data: BodyPOST) -> WithStatus<Json> {
    // business logic
    let response = HealthResponsePOST {
        name: data.name,
        age: data.age
    };

    reply_ok(&response)
}