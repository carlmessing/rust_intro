use std::net::SocketAddr;
use dotenv::dotenv;
use tracing_subscriber;
use utils::environment;
use crate::endpoints::oas;

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

#[tokio::main]
async fn main() {
    // Start server
    dotenv().ok();
    let ip = environment::ip();
    let port = environment::port();
    warp::serve(oas::routes())
        .run(SocketAddr::from((ip, port)))
        .await;
}
