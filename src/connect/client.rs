use log::{debug, info, trace};

use super::addrs;
use anyhow::{Context, Result};
use std::{
    net::{TcpListener, TcpStream, UdpSocket},
    thread,
    time::Duration,
};

pub struct Client {
    _server_conn: TcpStream,
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
        let udp_sock = UdpSocket::bind((std::net::Ipv4Addr::UNSPECIFIED, 0))
            .context("Couldn't bind to UDP socket")?;

        trace!("Joining multicast...");
        udp_sock
            .join_multicast_v4(&addrs::MULTICAST_IPV4, &std::net::Ipv4Addr::UNSPECIFIED)
            .context("Couldn't join multicast")?;

        trace!("Creating TcpListener to connect to server...");
        let listener = TcpListener::bind((
            std::net::Ipv4Addr::UNSPECIFIED,
            udp_sock
                .local_addr()
                .context("Couldn't get UDP Socket's local address.")?
                .port(),
        ))
        .context("Couldn't create listener")?;
        listener
            .set_nonblocking(true)
            .context("Can't make a non-blocking TcpListener")?;

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
                    debug!("2 seconds elapsed since connection request... Sending new one.");
                }
            }

            info!("Successfully connected to server!");

            accept_result.expect(
                "Since we just looped until accept_result was not an err, it must now be valid.",
            )
        };

        Ok(Self {
            _server_conn: server_conn,
        })
    }

    fn send_conn_request(udp_sock: &UdpSocket) {
        trace!("Sending server connection request...");
        udp_sock
            .send_to(&super::CONN_REQUEST, addrs::SOCKET_ADDR)
            .expect("Couldn't send connection request to server");
    }
}
