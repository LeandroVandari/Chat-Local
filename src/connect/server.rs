use super::{addrs, Message};
use log::{debug, error, info, trace};
use std::{
    net::{SocketAddr, TcpStream, UdpSocket},
    sync::Arc,
    thread::JoinHandle,
};

pub struct Server {
    udp_sock: Arc<UdpSocket>,
    connections: Vec<TcpStream>,
    buf: Vec<u8>,
    receive_messages_thread: Option<JoinHandle<()>>,
}

use anyhow::{Context, Result};

impl Server {
    /// Creates a [`Server`] that will bind an [`UdpSocket`] to [`addrs::SOCKET_ADDR`] and join the multicast at [`addrs::MULTICAST_IPV4`]. Note that the server won't actually listen to new connections until [`receive_connection`](Server::receive_connection) is called.
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
            udp_sock: Arc::new(udp_sock),
            connections,
            buf,
            receive_messages_thread: None,
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
    pub fn receive_connection(&mut self, addr: SocketAddr) -> Result<()> {
        info!("Ready to receive client connection...");

        if let std::result::Result::Ok((size, addr)) = self.udp_sock.recv_from(&mut self.buf) {
            if bincode::deserialize::<super::Message>(&self.buf[..size]).is_ok() {
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

    fn start_receive_messages(&mut self) {
        let udp_sock = self.udp_sock.clone();
        self.receive_messages_thread = Some(std::thread::spawn(move || {
            let mut msg_buf = vec![0; 1000];
            loop {
                if let std::result::Result::Ok((size, addr)) = udp_sock.recv_from(&mut msg_buf) {
                    match bincode::deserialize::<super::Message>(&msg_buf[..size]) {
                        Ok(message) => match message {
                            Message::Connection(message) => match message {
                                super::ConnectionMessage::ConnectionRequest => {
                                    info!("Received connection request from {addr}");
                                    match self.receive_connection(addr) {
                                        Ok(()) => debug!("Connected successfully to {addr}"),
                                        Err(e) => error!("Error connecting to {addr}: {e}"),
                                    }
                                }
                            },
                        },
                        Err(e) => debug!("Error deserializing message: {e}"),
                    }
                }
            }
        }))
    }
}
