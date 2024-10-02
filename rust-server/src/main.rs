use rust_server::Server;

fn main() {
    let mut server = Server::new();

    server.start("0.0.0.0:9000");
}