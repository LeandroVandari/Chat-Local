use std::cell::LazyCell;

use serde::{Deserialize, Serialize};

pub mod addrs;
pub mod client;
pub mod server;

/// Represents a request for the [`Server`](server::Server) to connect to. Contains a single [`u16`], which represents the port the [`Server`](server::Server) should request a connection to.
#[derive(Debug, Serialize, Deserialize)]
struct ConnectionRequest;

const CONN_REQUEST: LazyCell<Vec<u8>> = LazyCell::new(|| serde_json::to_string(&ConnectionRequest).unwrap().as_bytes().to_vec());
