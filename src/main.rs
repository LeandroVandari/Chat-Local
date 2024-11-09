use local::connect::server;

fn main() {
    env_logger::init();
    let mut serv = server::Server::new();

    loop {
        serv.receive_connection();
    }
}
