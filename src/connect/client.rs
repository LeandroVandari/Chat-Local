use super::addrs;
use std::net::{TcpListener, TcpStream, UdpSocket};

pub struct Client {
    server_conn: TcpStream,
}

impl Client {
    pub fn new() -> Self {
        let udp_sock = UdpSocket::bind((std::net::Ipv4Addr::UNSPECIFIED, 0))
            .expect("Couldn't create UDP Socket");

        udp_sock
            .join_multicast_v4(&addrs::MULTICAST_IPV4, &std::net::Ipv4Addr::UNSPECIFIED)
            .expect("Couldn't join multicast");

        let listener = TcpListener::bind((std::net::Ipv4Addr::UNSPECIFIED, 0))
            .expect("Couldn't create listener");
        let conn_request = super::ConnectionRequest::new(&listener);
        let request_json = serde_json::to_string(&conn_request).unwrap();

        udp_sock
            .send_to(request_json.as_bytes(), &addrs::SOCKET_ADDR)
            .expect("Couldn't send connection request to server");

        let (server_conn, _addr) = listener.accept().unwrap();

        Self { server_conn }
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
