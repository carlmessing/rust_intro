use crate::schemas::custom_errors::CommonError;

/// This function implements the business logic of the operation *squarer*.
/// It is called by requests to ```/square/{n}``` with method ``GET``.
/// All inputs are validated before passed into this function.
pub fn handler(base: i32) -> Result<i32, CommonError> {
    // business logic
    let result = base * base;
    Ok(result)
}