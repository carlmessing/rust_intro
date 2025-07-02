use std::panic;
use std::net::Ipv4Addr;

/// if set to `true`, no stacktrace is printed to `stdout` when the thread is panicking
pub fn print_stacktrace(stacktrace_allowed: bool) {
    if !stacktrace_allowed {
        panic::set_hook(Box::new(|_info| { }));
    }
}

/// retrieves the IP-Adress as a quadruple of the server from the `.env` file.
/// Defaults to [127.0.0.1](http://127.0.0.1) (localhost/loopback) if not set.
pub fn ip() -> [u8; 4] {
    let ip_str = std::env::var("SERVER_IP").unwrap_or_else(|_| "127.0.0.1".to_string());
    let ip: Ipv4Addr = ip_str.parse().unwrap_or_else(|_| { Ipv4Addr::new(127, 0, 0, 1) });
    ip.octets()
}

/// retrieves the IP-Adress as a quadruple of the server from the `.env` file.
/// Defaults to `3030` (default warp port) if not set.
pub fn port() -> u16 {
    let server_port: i32 = std::env::var("SERVER_PORT")
        .ok()
        .and_then(|val| val.parse::<i32>().ok())
        .unwrap_or(3030);
    server_port as u16
}