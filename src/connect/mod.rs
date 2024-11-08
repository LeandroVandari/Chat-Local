use std::net::TcpListener;

use serde::{Deserialize, Serialize};

pub mod addrs;
pub mod client;
pub mod server;

/// Represents a request for the [`Server`](server::Server) to connect to. Contains a single [`u16`], which represents the port the [`Server`](server::Server) should request a connection to.
#[derive(Debug, Serialize, Deserialize)]
struct ConnectionRequest(u16);

impl ConnectionRequest {
    pub fn new(listener: &TcpListener) -> Self {
        
        let port = listener.local_addr().unwrap().port();
        
        Self(port)
    }

    pub fn port(&self) -> u16{
        self.0
    }
}