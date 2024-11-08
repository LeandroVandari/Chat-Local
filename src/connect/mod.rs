use std::sync::LazyLock;

use serde::{Deserialize, Serialize};

pub mod addrs;
pub mod client;
pub mod server;

/// Represents a request for the [`Server`](server::Server) to connect to. Contains a single [`u16`], which represents the port the [`Server`](server::Server) should request a connection to.
#[derive(Debug, Serialize, Deserialize)]
struct ConnectionRequest;

static CONN_REQUEST: LazyLock<Vec<u8>> = LazyLock::new(|| {
    serde_json::to_string(&ConnectionRequest)
        .unwrap()
        .as_bytes()
        .to_vec()
});
