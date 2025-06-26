use crate::schemas::component_types::{Error, Operands, Result};

/// Contains the return value for the `POST /divide` handler.
pub(crate) enum ReturnValue {
    ResultOfAB(Result), // 200
    DivisionByZeroError (Error) // 400
    
}

/// This function implements the business logic of the operation *divider*.
/// It is called by requests to `/divide` with method `POST`.
/// All inputs are validated before being passed into this function.
pub fn handler(body: Operands) -> ReturnValue {
    // business logic
    if body.b == 0.0 {
        return ReturnValue::DivisionByZeroError(Error{ error: "b can not be zero".to_string() });
    }
    let result = body.a / body.b;
    ReturnValue::ResultOfAB(Result{ result })
}