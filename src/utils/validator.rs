use bytes::Bytes;
use serde::de::DeserializeOwned;
use warp::{Filter, Rejection};
use crate::endpoints;

pub fn json_body<T>() -> impl Filter<Extract = (T,), Error = Rejection> + Clone
where
    T: DeserializeOwned + Send + 'static,
{
    warp::body::bytes().and_then(|body: Bytes| async move {
        let res: Result<T, serde_json::Error> = serde_json::from_slice(&body);
        match res {
            Ok(data) => Ok(data),
            Err(e) => Err(warp::reject::custom(endpoints::BodyInputError {
                message: e.to_string(),
            })),
        }
    })
}

pub fn with_query<T>() -> impl Filter<Extract = (T,), Error = Rejection> + Clone
where
    T: DeserializeOwned + Send + 'static,
{
    warp::query::raw()
        .and_then(|raw: String| async move {
            match serde_urlencoded::from_str::<T>(&raw) {
                Ok(value) => Ok(value),
                Err(err) => {
                    let message = format!("Failed to parse query: {}", err);
                    Err(warp::reject::custom(endpoints::QueryInputError { message }))
                }
            }
        })
}
