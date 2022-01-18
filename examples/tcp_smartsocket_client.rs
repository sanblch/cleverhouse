use cleverhouse_by_akrutikov::devices::smartsocket::{stp::SmartSocketStream, tcp::SmartSocket};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let client = SmartSocketStream::connect("127.0.0.1:12345")?;
    let mut smartsocket = SmartSocket::new(client, String::from("a smart socket"));
    println!("{}", smartsocket.is_on());
    smartsocket.on();
    println!("{}", smartsocket.is_on());

    Ok(())
}
