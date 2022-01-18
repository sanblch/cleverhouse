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
        if let Err(e) = self.client.send(request) {
            println!("Send request error {:?}", e);
            return None;
        }
        let res = self.client.receive::<Response>();
        match res {
            Err(e) => {
                println!("Receive response error {:?}", e);
                None
            }
            Ok(response) => match response {
                Response::Error(str) => {
                    println!("Error occured in smart socket: {}", str);
                    None
                }
                _ => Some(response),
            },
        }
    }

    fn switch(&mut self, request: Request) -> Status {
        let res = self.send(request);
        match res {
            None => Status::Off,
            Some(response) => match response {
                Response::Status(status) => status,
                _ => {
                    println!("Unexpectedly receive power response");
                    Status::Off
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
            Status::On => true,
            Status::Off => false,
        }
    }

    pub fn off(&mut self) {
        match self.switch(Request::Off) {
            Status::On => println!("Smart socket couldn't switch off"),
            Status::Off => {}
        }
    }
    pub fn on(&mut self) {
        match self.switch(Request::On) {
            Status::On => {}
            Status::Off => println!("Smart socket couldn't switch on"),
        }
    }

    pub fn power(&mut self) -> f64 {
        let res = self.send(Request::Power);
        match res {
            None => NAN,
            Some(response) => match response {
                Response::Power(power) => power,
                _ => {
                    println!("Unexpectedly receive status response");
                    NAN
                }
            },
        }
    }

    pub fn shutdown(&mut self) {
        self.client.shutdown().unwrap();
    }
}
