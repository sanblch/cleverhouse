use std::error::Error;

use cleverhouse_by_akrutikov::devices::thermometer::server::Server;

fn main() -> Result<(), Box<dyn Error>> {
    let server = Server::bind("127.0.0.1:12345").expect("couldn't bind to address");

    server.run("127.0.0.1:54321");

    unreachable!();
    Ok(())
}
