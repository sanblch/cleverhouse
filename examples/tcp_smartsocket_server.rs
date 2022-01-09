use cleverhouse_by_akrutikov::devices::smartsocket::{
    api::{Request, Response, SmartSocketResult, Status},
    server::{SmartSocketServer, SmartSocketStream},
};
use std::error::Error;

fn handle_client(mut stream: SmartSocketStream) -> Result<(), Box<dyn Error>> {
    let res = stream.receive()?;
    println!("request = {:?}", res);
    match res {
        Request::On => {
            stream.send(SmartSocketResult::Ok(Response::On(Status::On)))?;
        }
        Request::Off => {}
        Request::Status => {}
        Request::Power => {}
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let str = "127.0.0.1:12345";
    let server = SmartSocketServer::bind(str)?;

    for stream in server.incoming() {
        handle_client(stream?)?;
        println!("Connection established!");
    }
    Ok(())
}
