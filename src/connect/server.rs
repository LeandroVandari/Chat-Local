use super::{addrs, Message};
use log::{debug, info, trace};
use serde::{Deserialize, Serialize};
use std::{
    io,
    net::{SocketAddr, TcpListener, TcpStream, UdpSocket},
    sync::{atomic::AtomicBool, Arc, Mutex},
    thread::JoinHandle,
};

pub struct Server {
    info: ServerInfo,
    udp_sock: Arc<UdpSocket>,
    connections: Arc<Mutex<Vec<TcpStream>>>,
    receive_messages_thread: Option<JoinHandle<()>>,
    shutdown: Arc<AtomicBool>,
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
        udp_sock
            .set_nonblocking(true)
            .context("Couldn't set udp socket to non-blocking...")?;
        trace!("Joining multicast on {}", addrs::MULTICAST_IPV4);
        udp_sock
            .join_multicast_v4(&addrs::MULTICAST_IPV4, &std::net::Ipv4Addr::UNSPECIFIED)
            .context("Couldn't join multicast")?;
        let connections = Arc::new(Mutex::new(Vec::new()));

        Ok(Self {
            // TODO: Allow user to change server name
            info: ServerInfo {
                name: String::from("ServerTest"),
                address: None,
                password_required: false,
            },
            udp_sock: Arc::new(udp_sock),
            connections,
            receive_messages_thread: None,
            shutdown: Arc::new(AtomicBool::new(false)),
        })
    }

    /// Starts a thread that will receive messages in the udp multicast and deal with them accordingly.
    pub fn start_receive_messages(&mut self) {
        let udp_sock = self.udp_sock.clone();
        let info = self.info.clone();
        let connections = self.connections.clone();
        let shutdown = self.shutdown.clone();
        self.receive_messages_thread = Some(
            std::thread::Builder::new()
                .name(String::from("receive_messages"))
                .spawn(move || {
                    let mut msg_buf = vec![0; 1000];
                    let tcp_connect = TcpListener::bind("0.0.0.0:0").unwrap();
                    tcp_connect.set_nonblocking(true).unwrap();
                    loop {
                        if shutdown.load(std::sync::atomic::Ordering::Relaxed) {
                            debug!("Shutdown flag set... Stopping receive_messages thread.");
                            break;
                        }
                        match udp_sock.recv_from(&mut msg_buf) {
                            std::result::Result::Ok((size, addr)) => {
                                match bincode::deserialize::<super::Message>(&msg_buf[..size]) {
                                    Ok(message) => match message {
                                        Message::Connection(message) => match message {
                                            super::ConnectionMessage::ServerList => {
                                                info!("Received server list from {addr}");
                                                udp_sock
                                                    .send_to(
                                                        &bincode::serialize(&info).unwrap(),
                                                        addrs::SOCKET_ADDR,
                                                    )
                                                    .unwrap();
                                            }
                                        },
                                    },
                                    Err(e) => debug!("Error deserializing message: {e}"),
                                }
                            }
                            Err(e) => {
                                if e.kind() != io::ErrorKind::WouldBlock {
                                    trace!(
                                "Received multicast message but it is *not* a connection request"
                            );
                                }
                            }
                        }
                        // Try to receive a client connection (since the listener is non-blocking it shouldn't interfere much in the time)
                        if let Ok((stream, addr)) = tcp_connect.accept() {
                            connections.lock().unwrap().push(stream);
                            debug!("Connected successfully to {addr}");
                        }
                    }
                })
                .unwrap(),
        )
    }
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ServerInfo {
    name: String,
    address: Option<SocketAddr>,
    password_required: bool,
}

impl Drop for Server {
    fn drop(&mut self) {
        self.shutdown
            .store(true, std::sync::atomic::Ordering::Relaxed);
        if let Some(thread) = self.receive_messages_thread.take() {
            thread.join().unwrap();
        }
    }
}
