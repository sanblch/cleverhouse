use cleverhouse_by_akrutikov::devices::smartsocket::r#async::server::Server;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut server = Server::bind("127.0.0.1:12345").await?;

    server.run().await;

    Ok(())
}
