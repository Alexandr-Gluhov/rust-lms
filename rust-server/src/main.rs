use rust_server::Server;

fn main() {
    let server = Server::new("0.0.0.0:80");

    server.start();
}