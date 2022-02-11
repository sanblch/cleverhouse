use cleverhouse_by_akrutikov::devices::smartsocket::r#async::{
    stp::SmartSocketStream, tcp::SmartSocket,
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = SmartSocketStream::connect("127.0.0.1:12345").await?;
    let mut smartsocket = SmartSocket::new(client, String::from("a smart socket"));
    println!("{}", smartsocket.is_on().await);
    smartsocket.on().await;
    println!("{}", smartsocket.is_on().await);

    Ok(())
}
