use cleverhouse_by_akrutikov::devices::smartsocket::api::{Request, SmartSocketResult};
use cleverhouse_by_akrutikov::devices::smartsocket::server::SmartSocketStream;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let str = "127.0.0.1:12345";
    let mut client = SmartSocketStream::connect(str)?;

    client.send(Request::On)?;

    let res = client.receive::<SmartSocketResult>()?;
    println!("response = {:?}", res);

    Ok(())
}
