use std::net::{TcpStream, UdpSocket};

pub struct Server {
    udp_sock: UdpSocket,
    connections: Vec<TcpStream>,
    buf: Vec<u8>,
}

impl Server {
    pub fn new() -> Self {
        let udp_sock = UdpSocket::bind(&super::addrs::SOCKED_ADDR).unwrap();
        udp_sock
            .join_multicast_v4(&super::addrs::IPV4, &std::net::Ipv4Addr::UNSPECIFIED)
            .unwrap();
        let connections = Vec::new();
        let buf = Vec::with_capacity(1000);

        Self {
            udp_sock,
            connections,
            buf,
        }
    }

    pub fn receive(&mut self) {
        println!("{:?}", self.udp_sock.recv_from(&mut self.buf));
    }
}
