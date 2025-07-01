use serde::Serialize;
use crate::endpoints::add::GetQueryParams;

/// Contains the return value for the `GET /add` handler.
#[derive(Serialize)]
#[serde(untagged)]
pub enum ReturnValue {
    SumOfAAndB {result: f64} // 200 response
}

/// This function implements the business logic of the operation *adder*.
/// It is called by requests to `/add` with method `GET`.
/// All inputs are validated before being passed into this function.
pub fn handler(params: GetQueryParams) -> ReturnValue {
    let result = (params.a + params.b) as f64;
    ReturnValue::SumOfAAndB {result}
}