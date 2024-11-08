use serde::{Deserialize, Serialize};

pub mod addrs;
pub mod client;
pub mod server;

/// Represents a request for the [`Server`](server::Server) to connect to. Contains a single [`u16`], which represents the port the [`Server`](server::Server) should request a connection to.
#[derive(Debug, Serialize, Deserialize)]
struct ConnectionRequest;
