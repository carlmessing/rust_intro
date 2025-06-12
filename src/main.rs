use warp::Filter;
use std::convert::Infallible;
use crate::endpoints::greet;
use crate::handlers::reply_notfound;

mod endpoints;
mod handlers;
mod utils;

async fn handle_notfound() -> Result<impl warp::Reply, Infallible> { Ok(reply_notfound()) }

#[tokio::main]
async fn main() {
    // health endpoint
    let health_get = warp::get()
        .and(warp::path!("health"))
        .and_then(endpoints::health::get);
    let health_post = warp::post()
        .and(warp::path!("health"))
        .and(warp::body::json())
        .and_then(endpoints::health::post);
    
    // greet endpoint
    let greet_post = warp::post()
        .and(warp::path!("greet" / String))
        .and(warp::body::bytes())
        .and_then(endpoints::greet::post);
    let greet_get = warp::get()
        .and(warp::path!("greet"))
        .and(warp::path::end())
        .and(warp::query::<greet::GetQueryParams>())
        .and_then(endpoints::greet::get)
        .recover(endpoints::greet::recover_get);
    
    // 404 endpoint
    let notfound = warp::any().and_then(handle_notfound);
    
    // Define routes
    let routes = health_get
        .or(health_post)
        .or(greet_get)
        .or(greet_post)
        .or(notfound);

    // Start server
    let ip = [127, 0, 0, 1];
    let port = 3030;
    println!("✅ Server is running on {}.{}.{}.{}:{}.", ip[0], ip[1], ip[2], ip[3], port);
    warp::serve(routes)
        .run((ip, port))
        .await;
}
