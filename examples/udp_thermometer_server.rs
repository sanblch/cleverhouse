use cleverhouse_by_akrutikov::devices::thermometer::server::Server;

fn main() {
    let server = Server::bind("127.0.0.1:12345").expect("couldn't bind to address");

    server.run("127.0.0.1:54321");
}
