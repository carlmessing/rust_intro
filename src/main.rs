use std::net::SocketAddr;
use dotenv::dotenv;
use tracing_subscriber;
use utils::environment;
use crate::endpoints::oas;

//new
use tracing_loki::Builder;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use tracing_subscriber::Registry;
use tracing_subscriber::prelude::*;
use url::Url;

mod endpoints;
mod handlers;
mod utils;
mod schemas;

fn init_tracing() {

    let loki_url = Url::parse("http://localhost:3100").expect("Invalid Loki URL");
    let builder = tracing_loki::builder();
    let (loki_layer, loki_task) = builder
        .build_url(loki_url)
        .expect("Failed to build Loki layer");


    // Fmt layer for stdout logs
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_thread_names(true);

    let env_filter = tracing_subscriber::EnvFilter::from_default_env()
        .add_directive("info".parse().unwrap());

    let subscriber = tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .with(loki_layer);

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");

    // Spawn background task to send logs to Loki
    tokio::spawn(loki_task);
}

#[tokio::main]
async fn main() {
    init_tracing();
    environment::print_stacktrace(false);

    // Start server
    dotenv().ok();
    let ip = environment::ip();
    let port = environment::port();
    warp::serve(oas::routes())
        .run(SocketAddr::from((ip, port)))
        .await;
}
