use std::net::TcpListener;

use log::warn;

pub fn default_or_available(port: u16) -> Option<u16> {
    if is_available(port) {
        Some(port)
    } else {
        warn!("Port {port} is not available, finding new port");
        get_available()
    }
}

pub fn get_available() -> Option<u16> {
    (3500..3999)
        .chain(4001..4999)
        .chain(5001..5999)
        .chain(8001..8999)
        .find(|port| is_available(*port))
}

pub fn is_available(port: u16) -> bool {
    TcpListener::bind(("127.0.0.1", port)).is_ok() && TcpListener::bind(("0.0.0.0", port)).is_ok()
}
