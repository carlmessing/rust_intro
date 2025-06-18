use warp::reject::InvalidQuery;
use crate::utils::{reply_invalid_parameters, reply_notfound};

pub mod health;
pub mod greet;

pub async fn recover_get(err: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(_) = err.find::<InvalidQuery>() { return Ok(reply_invalid_parameters()) }
    Err(warp::reject())
}