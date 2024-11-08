use super::addrs;
use log::{info, trace};
use std::net::{TcpStream, UdpSocket};

pub struct Server {
    udp_sock: UdpSocket,
    connections: Vec<TcpStream>,
    buf: Vec<u8>,
}

impl Server {
    pub fn new() -> Self {
        let udp_sock = UdpSocket::bind(addrs::SOCKET_ADDR).unwrap();
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

    pub fn receive_connections(&mut self) {
        if let Ok((size, addr)) = self.udp_sock.recv_from(&mut self.buf) {
             if   serde_json::from_slice::<super::ConnectionRequest>(&self.buf[..size]).is_ok()
            {
                trace!("Received connection request from {addr}");
                let client_conn =
                    TcpStream::connect(addr).expect("Couldn't connect to client");
                info!("Connected successfully to {addr}");

                self.connections.push(client_conn);
            } else {
                info!("Received multicast message but it is *not* a connection request");
            }
        }
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}
