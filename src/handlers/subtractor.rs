use crate::endpoints::subtract::GetQueryParams;
use crate::schemas::component_types::Result;

/// Contains the return value for the `GET /subtract` handler.
pub enum ReturnValue {
    ResultOfAB (Result) // 200
}

/// This function implements the business logic of the operation *subtractor*.
/// It is called by requests to ```/subtract``` with method ``GET``.
/// All inputs are validated before passed into this function.
pub fn handler(params: GetQueryParams) -> ReturnValue {
    let result = (params.a - params.b) as f64;
    ReturnValue::ResultOfAB(Result{ result })
}