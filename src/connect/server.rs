use super::addrs;
use log::{debug, info, trace};
use std::net::{TcpStream, UdpSocket};

pub struct Server {
    udp_sock: UdpSocket,
    connections: Vec<TcpStream>,
    buf: Vec<u8>,
}

use anyhow::{Context, Ok, Result};

impl Server {
    /// Creates a [`Server`] that will bind to an UDP Socket to [`addrs::SOCKET_ADDR`] and join the multicast at [`addrs::MULTICAST_IPV4`]. Note that the server won't actually listen to new connections until [`receive_connection`](Server::receive_connection) is called.
    ///
    /// ```
    /// use local::connect::Server;
    /// 
    /// let my_server = Server::new()?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    /// 
    /// # Errors
    /// This function will return [`Err`](anyhow::Result) anytime any of the networking code doesn't work. More specifically:
    /// 1. An UDP Socket can't be created and binded to [`addrs::SOCKET_ADDR`].
    /// 2. Can't [join a multicast](UdpSocket::join_multicast_v4).
    pub fn new() -> Result<Self> {
        trace!("Opening UDP socket...");
        let udp_sock = UdpSocket::bind(addrs::SOCKET_ADDR).context(format!(
            "Couldn't bind to UDP socket: {}",
            addrs::SOCKET_ADDR
        ))?;
        trace!("Joining multicast on {}", addrs::MULTICAST_IPV4);

        udp_sock
            .join_multicast_v4(&addrs::MULTICAST_IPV4, &std::net::Ipv4Addr::UNSPECIFIED)
            .context("Couldn't join multicast")?;
        let connections = Vec::new();
        let buf = vec![0; 1000];
        Ok(Self {
            udp_sock,
            connections,
            buf,
        })
    }

    /// Receive and accept a single connection request from the multicast.
    ///
    /// # Errors
    /// This will error if [`TcpStream`] can't connect to the client who requested the connection.
    /// 
    /// ```no_run
    /// use local::connect::Server;
    /// 
    /// let mut server =  Server::new()?;
    /// server.receive_connection();
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn receive_connection(&mut self) -> Result<()> {
        info!("Ready to receive client connection...");

        if let std::result::Result::Ok((size, addr)) = self.udp_sock.recv_from(&mut self.buf) {
            if serde_json::from_slice::<super::ConnectionRequest>(&self.buf[..size]).is_ok() {
                info!("Received connection request from {addr}");
                let client_conn = TcpStream::connect(addr).context("Couldn't connect to client")?;
                debug!("Connected successfully to {addr}");

                self.connections.push(client_conn);
            } else {
                trace!("Received multicast message but it is *not* a connection request");
            }
        }

        Ok(())
    }
}
