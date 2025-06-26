use crate::endpoints::add::GetQueryParams;
use crate::schemas::component_types::Result;

/// Contains the return value for the `GET /add` handler.
pub enum ReturnValue {
    SumOfAAndB (Result) // 200 response
}

/// This function implements the business logic of the operation *adder*.
/// It is called by requests to `/add` with method `GET`.
/// All inputs are validated before being passed into this function.
pub fn handler(params: GetQueryParams) -> ReturnValue {
    let result = (params.a + params.b) as f64;
    ReturnValue::SumOfAAndB (Result { result })
}