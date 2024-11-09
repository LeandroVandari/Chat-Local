use local::connect::server;
use log::{debug, error};
fn main() {
    env_logger::init();

    let mut server = match server::Server::new() {
        Ok(server) => {
            debug!("Created server successfully.");
            server
        }
        Err(e) => {
            error!("Couldn't create server: {e}. Exiting...");
            panic!("Couldn't create server due to error: {e}. Exiting...");
        }
    };

    loop {
        if let Err(e) = server.receive_connection() {
            debug!("Couldn't connect to client who requested connection: {e}");
        }
    }
}
