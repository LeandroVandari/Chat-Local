use std::cell::LazyCell;

use serde::{Deserialize, Serialize};

pub mod addrs;
pub mod client;
pub mod server;

/// Represents a request for the [`Server`](server::Server) to connect to. Contains a single [`u16`], which represents the port the [`Server`](server::Server) should request a connection to.
#[derive(Debug, Serialize, Deserialize)]
struct ConnectionRequest;

const CONN_REQUEST: LazyCell<Vec<u8>> = LazyCell::new(|| serde_json::to_string(&ConnectionRequest).unwrap().as_bytes().to_vec());


#[cfg(test)]
mod tests {
    use std::net::UdpSocket;

    #[test]
    fn can_bind_to_udp() {
        UdpSocket::bind(super::addrs::SOCKET_ADDR).unwrap();
    }

    #[test]
    fn can_join_multicast() {
        let sock = UdpSocket::bind(super::addrs::SOCKET_ADDR).unwrap();
        sock
            .join_multicast_v4(&super::addrs::MULTICAST_IPV4, &std::net::Ipv4Addr::UNSPECIFIED)
            .unwrap();
    }
}