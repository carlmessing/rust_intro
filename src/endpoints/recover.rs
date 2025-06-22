use warp::{Rejection, Reply, http::StatusCode};
use tracing::warn;

pub async fn recover(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    warn!("Recover triggered: {:?}", err);

    if err.is_not_found() {
        Ok(warp::reply::with_status(
            "Not Found",
            StatusCode::NOT_FOUND,
        ))
    } else if let Some(_) = err.find::<warp::filters::query::InvalidQuery>() {
        Ok(warp::reply::with_status(
            "Invalid query parameters",
            StatusCode::BAD_REQUEST,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Unhandled rejection",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}
