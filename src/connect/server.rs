use std::net::{TcpStream, UdpSocket};
use super::addrs;
use log::info;

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
        let buf = vec![0;1000];

        Self {
            udp_sock,
            _connections,
            buf,
        }
    }

    pub fn receive_connections(&mut self) {
        if let Ok((size, addr)) = self.udp_sock.recv_from(&mut self.buf) {
            info!("Received message in multicast from {addr}: {}...", std::str::from_utf8(&self.buf[..size]).expect("Valid UTF-8 from device"));
        }
    }
}


impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}