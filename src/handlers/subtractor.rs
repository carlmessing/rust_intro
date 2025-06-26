use crate::endpoints::subtract::GetQueryParams;
use crate::schemas::custom_errors::CommonError;

/// This function implements the business logic of the operation *subtractor*.
/// It is called by requests to ```/subtract``` with method ``GET``.
/// All inputs are validated before passed into this function.
pub fn handler(params: GetQueryParams) -> Result<i32, CommonError> {
    let result = params.a - params.b;
    Ok(result)
}