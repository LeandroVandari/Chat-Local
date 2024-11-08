use super::addrs;
use log::info;
use std::net::{TcpStream, UdpSocket};

pub struct Server {
    udp_sock: UdpSocket,
    _connections: Vec<TcpStream>,
    buf: Vec<u8>,
}

impl Server {
    pub fn new() -> Self {
        let udp_sock = UdpSocket::bind(addrs::SOCKET_ADDR).unwrap();
        udp_sock
            .join_multicast_v4(&addrs::MULTICAST_IPV4, &std::net::Ipv4Addr::UNSPECIFIED)
            .unwrap();
        let _connections = Vec::new();
        let buf = vec![0; 1000];

        Self {
            udp_sock,
            _connections,
            buf,
        }
    }

    pub fn receive_connections(&mut self) {
        if let Ok((size, addr)) = self.udp_sock.recv_from(&mut self.buf) {
            let msg = &self.buf[..size];
            if msg == super::CONN_REQUEST {
                info!("Received new connection request in multicast from {addr}.")
            } else {
                info!("Received message in multicast, but it wasn't a connection request...")
            }
        }
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}
