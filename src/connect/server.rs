use std::net::{TcpStream, UdpSocket};
use super::addrs;

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
        let buf = Vec::with_capacity(1000);

        Self {
            udp_sock,
            _connections,
            buf,
        }
    }

    pub fn receive(&mut self) {
        println!("{:?}", self.udp_sock.recv_from(&mut self.buf));
    }
}


impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}