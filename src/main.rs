use std::convert::Infallible;
use std::env;
use std::net::{Ipv4Addr, SocketAddr};
use tracing_subscriber;
use utils::reply_notfound;
use warp::Filter;

mod endpoints;
mod handlers;
mod utils;

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_target(true) // show which module logs come from
        .with_thread_names(true) // show thread names
        .with_env_filter("info") // log level (can be overridden with RUST_LOG)
        .init();
}

async fn handle_notfound() -> Result<impl warp::Reply, Infallible> { Ok(reply_notfound()) }

async fn env_ip() -> [u8; 4] {
    let ip_str = env::var("SERVER_IP").unwrap_or_else(|_| "127.0.0.1".to_string());
    let ip: Ipv4Addr = ip_str.parse().unwrap_or_else(|_| { Ipv4Addr::new(127, 0, 0, 1) });
    ip.octets()
}

async fn env_port() -> u16 {
    let server_port: i32 = env::var("SERVER_PORT")
        .ok()
        .and_then(|val| val.parse::<i32>().ok())
        .unwrap_or(3030);
    server_port as u16
}

#[tokio::main]
async fn main() {
    // init tracing
    init_tracing();

    // health endpoint
    let health_get = warp::get()
        .and(warp::path!("health"))
        .and_then(endpoints::health::get);
    let health_post = warp::post()
        .and(warp::path!("health"))
        .and(warp::body::json())
        .and_then(endpoints::health::post);
    let health = health_post.or(health_get);
    
    // greet endpoint
    let greet_post = warp::post()
        .and(warp::path!("greet" / String))
        .and(warp::body::bytes())
        .and_then(endpoints::greet::post);
    let greet_get = warp::get()
        .and(warp::path!("greet"))
        .and(warp::path::end())
        .and(warp::query::<endpoints::greet::GetQueryParams>())
        .and_then(endpoints::greet::get);
    let greet = greet_post.or(greet_get).recover(endpoints::recover_get);

    // 404 endpoint
    let notfound = warp::any().and_then(handle_notfound);
    
    // Define routes
    let routes = greet
        .or(health)
        .or(notfound)
        .with(warp::log("api"));

    // Start server
    let ip = env_ip().await;
    let port = env_port().await;
    println!("✅ Server is running on {}.{}.{}.{}:{}.", ip[0], ip[1], ip[2], ip[3], port);
    warp::serve(routes)
        .run(SocketAddr::from((ip, port)))
        .await;
}
