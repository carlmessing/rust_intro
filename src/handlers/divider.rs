use crate::schemas::component_types::{Error, Operands};

/// This function implements the business logic of the operation *divider*.
/// It is called by requests to /divide with method POST.
/// All inputs are validated before being passed into this function.
pub fn handler(body: Operands) -> Result<i32, Error> {
    // business logic
    if body.b == 0 {
        return Err(Error{error: "b can not be zero".to_string()})
    }
    let result = body.a / body.b;
    Ok(result)
}