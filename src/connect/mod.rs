//! The module that handles the low-level connection details. Requires a [`Server`] and [`Client`]s that will connect to that server. The [`Client`]s discover the server automatically through multicasting.

use std::sync::LazyLock;

use serde::{Deserialize, Serialize};

pub mod addrs;
mod client;
mod server;

pub use client::Client;
pub use server::Server;
/*
static CONN_REQUEST: LazyLock<Vec<u8>> = LazyLock::new(|| {
    bincode::serialize(&Message::Connection(ConnectionMessage::ConnectionRequest)).unwrap()
}); */

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
    ServerInfo(server::ServerInfo), // ConnectionRequest,
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

    #[test]
    fn can_set_sock_to_nonblocking() {
        let sock = UdpSocket::bind(super::addrs::SOCKET_ADDR).unwrap();
        sock.join_multicast_v4(
            &super::addrs::MULTICAST_IPV4,
            &std::net::Ipv4Addr::UNSPECIFIED,
        )
        .unwrap();
        assert!(sock.set_nonblocking(true).is_ok())
    }

    #[test]
    fn can_send_message_in_multicast() {
        let sock = UdpSocket::bind(super::addrs::SOCKET_ADDR).unwrap();
        sock.join_multicast_v4(
            &super::addrs::MULTICAST_IPV4,
            &std::net::Ipv4Addr::UNSPECIFIED,
        )
        .unwrap();
        assert!(sock
            .send_to(&super::SERVER_LIST, super::addrs::SOCKET_ADDR)
            .is_ok())
    }

    #[test]
    fn can_receive_message_in_multicast() {
        let mut buf = [0; 1000];
        let sock = UdpSocket::bind(super::addrs::SOCKET_ADDR).unwrap();
        sock.join_multicast_v4(
            &super::addrs::MULTICAST_IPV4,
            &std::net::Ipv4Addr::UNSPECIFIED,
        )
        .unwrap();
        sock.set_nonblocking(true).unwrap();

        let recv_result = sock.recv_from(&mut buf);

        match recv_result {
            Ok(_) => (),
            Err(e) => assert!(matches!(e.kind(), std::io::ErrorKind::WouldBlock)),
        }
    }
}
