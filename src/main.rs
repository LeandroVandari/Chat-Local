use local::connect::server;

fn main() {
    let mut serv = server::Server::new();

    loop {
        serv.receive();
    }
}
