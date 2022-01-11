use std::f64::NAN;

use super::{
    api::{Request, Response, Status},
    stp::SmartSocketStream,
};
use crate::device::Device;

pub struct SmartSocket {
    pub description: String,
    pub client: SmartSocketStream,
}

impl Device for SmartSocket {
    fn description(&self) -> &str {
        &*self.description
    }
}

impl SmartSocket {
    fn send(&mut self, request: Request) -> Option<Response> {
        match self.client.send(request) {
            Err(e) => {
                println!("Send request error {:?}", e);
                return None;
            }
            _ => {}
        }
        let res = self.client.receive::<Response>();
        match res {
            Err(e) => {
                println!("Receive response error {:?}", e);
                return None;
            }
            Ok(response) => match response {
                Response::Error(str) => {
                    println!("Error occured in smart socket: {}", str);
                    return None;
                }
                _ => Some(response),
            },
        }
    }

    fn switch(&mut self, request: Request) -> Status {
        let res = self.send(request);
        match res {
            None => return Status::Off,
            Some(response) => match response {
                Response::Status(status) => status,
                _ => {
                    println!("Unexpectedly receive power response");
                    return Status::Off;
                }
            },
        }
    }

    pub fn new(client: SmartSocketStream, description: String) -> Self {
        SmartSocket {
            description,
            client,
        }
    }

    pub fn is_on(&mut self) -> bool {
        match self.switch(Request::Status) {
            Status::On => return true,
            Status::Off => return false,
        }
    }

    pub fn on(&mut self) {
        match self.switch(Request::On) {
            Status::On => println!("Smart socket couldn't switch off"),
            Status::Off => {}
        }
    }
    pub fn off(&mut self) {
        match self.switch(Request::Off) {
            Status::On => {}
            Status::Off => println!("Smart socket couldn't switch on"),
        }
    }

    pub fn power(&mut self) -> f64 {
        let res = self.send(Request::Power);
        match res {
            None => return NAN,
            Some(response) => match response {
                Response::Power(power) => power,
                _ => {
                    println!("Unexpectedly receive status response");
                    return NAN;
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smartsocket() {
        let client = SmartSocketStream::connect("127.0.0.1:12345").unwrap();
        let _smartsocket = SmartSocket::new(client, String::from("a smart socket"));
    }
}
