use cleverhouse_by_akrutikov::devices::smartsocket::server::Server;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut server = Server::bind("127.0.0.1:12345")?;

    server.run();

    Ok(())
}
