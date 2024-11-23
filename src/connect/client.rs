use log::{debug, info, trace};

use crate::connect::SERVER_LIST;

use super::{addrs, server::ServerInfo};
use anyhow::{Context, Result};
use std::{
    net::{TcpStream, UdpSocket},
    sync::{atomic::AtomicBool, Arc, Mutex},
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

pub struct Client {
    server_conn: Option<TcpStream>,
    server_list: ServerList,
}

impl Client {
    #[allow(
        clippy::missing_panics_doc,
        reason = "The only possible panic here is something that will never panic"
    )]
    /// Creates a new [`Client`]. It joins the multicast address through an [`UdpSocket`], and sends requests to connect to a server until one answers.
    ///
    /// ```no_run
    /// use local::connect::Client;
    ///
    /// let my_client = Client::new()?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    ///
    /// # Errors
    /// This function will return [`Err`](anyhow::Result) anytime any of the networking code doesn't work. More specifically:
    /// 1. An UDP Socket can't be created and binded to [`std::net::Ipv4Addr::UNSPECIFIED`].
    /// 2. Can't [join a multicast](UdpSocket::join_multicast_v4).
    /// 3. Can't create a [`TcpListener`] and bind it to [`std::net::Ipv4Addr::UNSPECIFIED`].
    /// 4. Can't call [`TcpListener::set_nonblocking(true)`](TcpListener::set_nonblocking)
    pub fn new() -> Result<Self> {
        trace!("Binding to UDP socket...");
        let multicast = UdpSocket::bind((std::net::Ipv4Addr::UNSPECIFIED, 0))
            .context("Couldn't bind to UDP socket")?;

        trace!("Joining multicast...");
        multicast
            .join_multicast_v4(&addrs::MULTICAST_IPV4, &std::net::Ipv4Addr::UNSPECIFIED)
            .context("Couldn't join multicast")?;

        let server_list = ServerList::new(multicast);
        Ok(Self {
            server_conn: None,
            server_list,
        })
    }

    /*     fn send_conn_request(udp_sock: &UdpSocket) {
        trace!("Sending server connection request...");
        udp_sock
            .send_to(&super::CONN_REQUEST, addrs::SOCKET_ADDR)
            .expect("Couldn't send connection request to server");
    } */

    fn send_server_list(udp_sock: &UdpSocket) {
        trace!("Sending server list request...");
        udp_sock
            .send_to(&SERVER_LIST, addrs::SOCKET_ADDR)
            .expect("Couldn't request server list");
    }
}

struct ServerList {
    multicast: Arc<UdpSocket>,
    servers: Arc<Mutex<Vec<ServerInfo>>>,
    list_thread: Option<JoinHandle<()>>,
    shutdown: Arc<AtomicBool>,
}

impl ServerList {
    pub fn new(multicast: UdpSocket) -> Self {
        let multicast = Arc::new(multicast);
        multicast.set_nonblocking(true).unwrap();
        let servers = Arc::new(Mutex::new(Vec::new()));
        let shutdown = Arc::new(AtomicBool::new(false));

        Client::send_server_list(&multicast);

        let t_multicast = multicast.clone();
        let t_servers = servers.clone();
        let t_shutdown = shutdown.clone();

        let list_thread = thread::Builder::new()
            .name(String::from("server_list"))
            .spawn(move || {
                let mut time = std::time::Instant::now();
                let mut buf = vec![0; 1024];
                loop {
                    if t_shutdown.load(std::sync::atomic::Ordering::Relaxed) {
                        debug!("Dropping ServerList... Closing server_list thread");
                        break;
                    }
                    thread::sleep(Duration::from_millis(500));
                    if time.elapsed() > Duration::from_secs(5) {
                        time = Instant::now();
                        trace!("5 secs since last server list, sending new one...");
                        Client::send_server_list(&t_multicast);
                    }
                    if let Ok((size, addr)) = t_multicast.recv_from(&mut buf) {
                        info!("Received message from {addr} on multicast.");
                        if let Ok(info) = bincode::deserialize::<ServerInfo>(&buf[..size]) {
                            debug!("Message was server info. Logging it to trace...");
                            info!("Server info received: {info:?}");
                            t_servers.lock().unwrap().push(info);
                        }
                    }
                }
            })
            .unwrap();

        Self {
            multicast,
            servers,
            list_thread: Some(list_thread),
            shutdown,
        }
    }
}

impl Drop for ServerList {
    fn drop(&mut self) {
        self.shutdown
            .store(true, std::sync::atomic::Ordering::Relaxed);
        if let Some(thread) = self.list_thread.take() {
            thread.join().unwrap();
        }
    }
}
