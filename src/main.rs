use warp::Filter;
use std::convert::Infallible;
use std::env;
use std::net::{Ipv4Addr, SocketAddr};
use crate::endpoints::greet;
use crate::handlers::reply_notfound;

mod endpoints;
mod handlers;
mod utils;

async fn handle_notfound() -> Result<impl warp::Reply, Infallible> { Ok(reply_notfound()) }

async fn env_ip() -> [u8; 4] {
    let ip_str = env::var("SERVER_IP").unwrap_or_else(|_| "127.0.0.1".to_string());

    let ip: Ipv4Addr = ip_str.parse().unwrap_or_else(|_| {
        Ipv4Addr::new(127, 0, 0, 1)
    });
    // [ip.octets()[0] as i32, ip.octets()[1] as i32, ip.octets()[2] as i32, ip.octets()[3] as i32]
    ip.octets()
}

async fn env_port() -> u16 {
    let my_value: i32 = env::var("SERVER_PORT")
        .ok()
        .and_then(|val| val.parse::<i32>().ok())
        .unwrap_or(3030);
    my_value as u16
}

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
    let ip = env_ip().await;
    let port = env_port().await;
    println!("✅ Server is running on {}.{}.{}.{}:{}.", ip[0], ip[1], ip[2], ip[3], port);
    warp::serve(routes)
        .run(SocketAddr::from((ip, port)))
        .await;
}
