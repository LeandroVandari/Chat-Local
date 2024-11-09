use super::addrs;
use log::{debug, info, trace};
use std::net::{TcpStream, UdpSocket};

pub struct Server {
    udp_sock: UdpSocket,
    connections: Vec<TcpStream>,
    buf: Vec<u8>,
}

impl Server {
    pub fn new() -> Self {
        trace!("Opening UDP socket...");
        let udp_sock = UdpSocket::bind(addrs::SOCKET_ADDR).unwrap();
        trace!("Joining multicast on {}", addrs::MULTICAST_IPV4);
        udp_sock
            .join_multicast_v4(&addrs::MULTICAST_IPV4, &std::net::Ipv4Addr::UNSPECIFIED)
            .unwrap();
        let connections = Vec::new();
        let buf = vec![0; 1000];

        Self {
            udp_sock,
            connections,
            buf,
        }
    }

    pub fn receive_connection(&mut self) {
        info!("Ready to receive client connection...");
        if let Ok((size, addr)) = self.udp_sock.recv_from(&mut self.buf) {
            if serde_json::from_slice::<super::ConnectionRequest>(&self.buf[..size]).is_ok() {
                info!("Received connection request from {addr}");
                let client_conn = TcpStream::connect(addr).expect("Couldn't connect to client");
                debug!("Connected successfully to {addr}");

                self.connections.push(client_conn);
            } else {
                trace!("Received multicast message but it is *not* a connection request");
            }
        }
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}
