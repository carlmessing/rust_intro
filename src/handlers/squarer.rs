use crate::schemas::component_types::Result;

/// Contains the return value for the `GET /square/{n}` handler.
pub enum ReturnValue {
    ResultOfNSquared (Result) // 200
}

/// This function implements the business logic of the operation *squarer*.
/// It is called by requests to ```/square/{n}``` with method ``GET``.
/// All inputs are validated before passed into this function.
pub fn handler(base: i32) -> ReturnValue {
    // business logic
    let result = (base * base) as f64;
    ReturnValue::ResultOfNSquared(Result{ result })
}