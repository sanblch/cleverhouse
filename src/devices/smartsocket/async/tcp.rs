use std::f64::NAN;

use crate::devices::smartsocket::api::{Request, Response, Status};

use super::stp::SmartSocketStream;
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
    async fn send(&mut self, request: Request) -> Option<Response> {
        if let Err(e) = self.client.send(request).await {
            println!("Send request error {:?}", e);
            return None;
        }
        let res = self.client.receive::<Response>().await;
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

    async fn switch(&mut self, request: Request) -> Status {
        let res = self.send(request).await;
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

    pub async fn is_on(&mut self) -> bool {
        match self.switch(Request::Status).await {
            Status::On => true,
            Status::Off => false,
        }
    }

    pub async fn off(&mut self) {
        match self.switch(Request::Off).await {
            Status::On => println!("Smart socket couldn't switch off"),
            Status::Off => {}
        }
    }
    pub async fn on(&mut self) {
        match self.switch(Request::On).await {
            Status::On => {}
            Status::Off => println!("Smart socket couldn't switch on"),
        }
    }

    pub async fn power(&mut self) -> f64 {
        let res = self.send(Request::Power).await;
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
}
