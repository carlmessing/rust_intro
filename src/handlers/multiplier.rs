use crate::schemas::component_types::{Operands, Result};

/// Contains the return value for the `POST /multiply` handler.
pub enum ReturnValue {
    ProductOfAAndB (Result)
}

/// This function implements the business logic of the operation *multiplier*.
/// It is called by requests to ```/multiply``` with method ``POST``.
/// All inputs are validated before passed into this function.
pub fn handler(body: Operands) -> ReturnValue {
    // business logic
    let result = body.a * body.b;
    ReturnValue::ProductOfAAndB( Result{ result })
}