use std::env;
use std::net::{Ipv4Addr, SocketAddr};
use tracing_subscriber;
use tracing_subscriber::filter::FilterExt;
use warp::Filter;
use crate::utils::validator::json_body;

mod endpoints;
mod handlers;
mod utils;
mod schemas;

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_target(true) // show which module logs come from
        .with_thread_names(true) // show thread names
        .with_env_filter("info") // log level (can be overridden with RUST_LOG)
        .init();
}

fn env_ip() -> [u8; 4] {
    let ip_str = env::var("SERVER_IP").unwrap_or_else(|_| "127.0.0.1".to_string());
    let ip: Ipv4Addr = ip_str.parse().unwrap_or_else(|_| { Ipv4Addr::new(127, 0, 0, 1) });
    ip.octets()
}

fn env_port() -> u16 {
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
    
    // add endpoint
    let add_get = warp::get()
        .and(warp::path!("add"))
        .and(warp::path::end())
        .and(warp::query::<endpoints::add::GetQueryParams>())
        .and_then(endpoints::add::get)
        .boxed();
    let add = add_get;
    
    // subtract endpoint
    let subtract_get = warp::get()
        .and(warp::path!("subtract"))
        .and(warp::path::end())
        .and(warp::query::<endpoints::subtract::GetQueryParams>())
        .and_then(endpoints::subtract::get)
        .boxed();
    let subtract = subtract_get;
    
    // multiply endpoint
    let multiply_post = warp::post()
        .and(warp::path!("multiply"))
        .and(warp::path::end())
        .and(json_body::<schemas::multiply::MultiplyBodyPOST>())
        .and_then(endpoints::multiply::post)
        .boxed();
    let multiply = multiply_post;
    
    // divide endpoint
    let divide_post = warp::post()
        .and(warp::path!("divide"))
        .and(warp::path::end())
        .and(json_body::<schemas::divide::DivideBodyPOST>())
        .and_then(endpoints::divide::post)
        .boxed();
    let divide = divide_post;
    
    // square endpoint
    let square_get = warp::get()
        .and(warp::path!("square" / i32))
        .and(warp::path::end())
        .and_then(endpoints::square::get)
        .boxed();
    let square = square_get;
    
    // kubernetes health check endpoints
    let livez = warp::get()
        .and(warp::path!("healthcheck" / "livez"))
        .and(warp::path::end())
        .and_then(endpoints::health::livez)
        .boxed();
    let readyz = warp::get()
        .and(warp::path!("healthcheck" / "readyz"))
        .and(warp::path::end())
        .and_then(endpoints::health::readyz)
        .boxed();
    let healthcheck = livez.or(readyz);
    
    // Define routes
    let routes = add
        .or(subtract)
        .or(multiply)
        .or(divide)
        .or(square)
        .or(healthcheck)
        .boxed()
        .recover(endpoints::recover)
        .with(warp::log("api"));

    // Start server
    let ip = env_ip();
    let port = env_port();
    println!("✅ Server is running on {}.{}.{}.{}:{}.", ip[0], ip[1], ip[2], ip[3], port);
    warp::serve(routes)
        .run(SocketAddr::from((ip, port)))
        .await;
}
