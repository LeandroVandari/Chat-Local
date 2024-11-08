use log::{debug, info, trace};

use super::addrs;
use std::{
    net::{TcpListener, TcpStream, UdpSocket},
    thread,
    time::Duration,
};

pub struct Client {
    _server_conn: TcpStream,
}

impl Client {
    pub fn new() -> Self {
        trace!("Binding to UDP socket...");
        let udp_sock = UdpSocket::bind((std::net::Ipv4Addr::UNSPECIFIED, 0))
            .expect("Couldn't create UDP Socket");

        trace!("Joining multicast...");
        udp_sock
            .join_multicast_v4(&addrs::MULTICAST_IPV4, &std::net::Ipv4Addr::UNSPECIFIED)
            .expect("Couldn't join multicast");

        trace!("Creating TcpListener to connect to server...");
        let listener = TcpListener::bind((
            std::net::Ipv4Addr::UNSPECIFIED,
            udp_sock.local_addr().unwrap().port(),
        ))
        .expect("Couldn't create listener");
        listener
            .set_nonblocking(true)
            .expect("Can't make a non-blocking TcpListener");

        Self::send_conn_request(&udp_sock);
        let mut before_accept = std::time::Instant::now();
        let (server_conn, _addr) = {
            let mut accept_result = listener.accept();
            while accept_result.is_err() {
                thread::sleep(Duration::from_millis(10));
                accept_result = listener.accept();
                if before_accept.elapsed() > Duration::from_secs(2) {
                    Self::send_conn_request(&udp_sock);
                    before_accept = std::time::Instant::now();
                    debug!("2 seconds elapsed since connection request... Sending new one.")
                }
            }
            info!("Successfully connected to server!");
            accept_result.unwrap()
        };

        Self {
            _server_conn: server_conn,
        }
    }

    fn send_conn_request(udp_sock: &UdpSocket) {
        trace!("Sending server connection request...");
        udp_sock
            .send_to(&super::CONN_REQUEST, addrs::SOCKET_ADDR)
            .expect("Couldn't send connection request to server");
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
