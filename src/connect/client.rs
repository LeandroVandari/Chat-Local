use std::net::UdpSocket;
use super::addrs;

pub struct Client {
    udp_sock: UdpSocket
}

impl Client {
    pub fn new() -> Self {
        let udp_sock = UdpSocket::bind((std::net::Ipv4Addr::UNSPECIFIED, 0)).expect("Couldn't create UDP Socket");

        udp_sock.join_multicast_v4(&addrs::MULTICAST_IPV4, &std::net::Ipv4Addr::UNSPECIFIED).expect("Couldn't join multicast");


        udp_sock.send_to(b"Hi", addrs::SOCKET_ADDR).unwrap();

        Self { udp_sock }
    }
}

impl  Default for  Client {
    fn default() -> Self {
        Self::new()
    }
}