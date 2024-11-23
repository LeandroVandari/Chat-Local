use std::hint::spin_loop;

use local::connect::Client;
use log::{debug, error};
fn main() {
    env_logger::init();
    #[cfg(feature = "server")]
    let mut server = match local::connect::Server::new() {
        Ok(server) => {
            debug!("Created server successfully.");
            server
        }
        Err(e) => {
            error!("Couldn't create server: {e}. Exiting...");
            panic!("Couldn't create server due to error: {e}. Exiting...");
        }
    };
    #[cfg(feature = "server")]
    server.start_receive_messages();

    #[cfg(feature = "client")]
    let _client = match Client::new() {
        Ok(client) => {
            debug!("Created client successfully.");
            client
        }
        Err(e) => {
            error!("Couldn't create client: {e}. Exiting...");
            panic!("Couldn't create client due to error: {e}. Exiting...");
        }
    };

    loop {
        spin_loop();
    }
}
