//! The module that handles the low-level connection details. Requires a [`Server`] and [`Client`]s that will connect to that server. The [`Client`]s discover the server automatically through multicasting.

use std::sync::LazyLock;

use serde::{Deserialize, Serialize};

pub mod addrs;
mod client;
mod server;

pub use client::Client;
pub use server::Server;

static CONN_REQUEST: LazyLock<Vec<u8>> = LazyLock::new(|| {
    bincode::serialize(&Message::Connection(ConnectionMessage::ConnectionRequest)).unwrap()
});

static SERVER_LIST: LazyLock<Vec<u8>> = LazyLock::new(|| {
    bincode::serialize(&Message::Connection(ConnectionMessage::ServerList)).unwrap()
});

#[derive(Debug, Serialize, Deserialize)]
enum Message {
    Connection(ConnectionMessage),
}

#[derive(Debug, Serialize, Deserialize)]
enum ConnectionMessage {
    ServerList,
    ConnectionRequest,
}

#[cfg(test)]
mod tests {
    use std::net::UdpSocket;

    #[test]
    fn can_bind_to_udp() {
        assert!(UdpSocket::bind(super::addrs::SOCKET_ADDR).is_ok());
    }

    #[test]
    fn can_join_multicast() {
        let sock = UdpSocket::bind(super::addrs::SOCKET_ADDR).unwrap();
        assert!(sock
            .join_multicast_v4(
                &super::addrs::MULTICAST_IPV4,
                &std::net::Ipv4Addr::UNSPECIFIED,
            )
            .is_ok())
    }
}
