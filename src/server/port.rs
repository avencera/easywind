use std::net::TcpListener;

pub fn default_or_available(port: u16) -> Option<u16> {
    if is_available(port) {
        Some(port)
    } else {
        get_available()
    }
}

pub fn get_available() -> Option<u16> {
    (3000..5000)
        .chain(8000..9000)
        .find(|port| is_available(*port))
}

pub fn is_available(port: u16) -> bool {
    TcpListener::bind(("127.0.0.1", port)).is_ok()
}
