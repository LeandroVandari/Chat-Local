pub mod client;
pub mod server;
pub mod addrs;

const CONN_REQUEST: &'static [u8] = "REQ CONN".as_bytes();