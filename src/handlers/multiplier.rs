use crate::schemas::custom_errors::CommonError;
use crate::schemas::component_types::Operands;

/// This function implements the business logic of the operation *multiplier*.
/// It is called by requests to ```/multiply``` with method ``POST``.
/// All inputs are validated before passed into this function.
pub fn handler(body: Operands) -> Result<i32, CommonError> {
    // business logic
    let result = body.a * body.b;
    Ok(result)
}