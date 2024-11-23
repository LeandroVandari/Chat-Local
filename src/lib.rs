pub mod connect;

#[cfg(all(feature = "server", feature = "client"))]
compile_error!("Can't have both the client and server features on");
