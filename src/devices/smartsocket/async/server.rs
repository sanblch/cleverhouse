use std::f64::NAN;

use rand::Rng;
use tokio::net::ToSocketAddrs;

use crate::devices::smartsocket::api::{Request, Response, Status};

use super::stp::{SmartSocketServer, SmartSocketStream};

pub struct Server {
    server: SmartSocketServer,
    status: Status,
    power: f64,
}

impl Server {
    pub async fn bind<A: ToSocketAddrs>(addr: A) -> std::io::Result<Server> {
        let res = SmartSocketServer::bind(addr).await?;
        let server = Server {
            server: res,
            status: Status::Off,
            power: NAN,
        };
        Ok(server)
    }

    pub async fn run(&mut self) {
        loop {
            let stream_res = self.server.accept().await;
            match stream_res {
                Err(e) => {
                    println!("Error occured on stream creation: {:?}", e);
                    continue;
                }
                Ok(mut stream) => loop {
                    match stream.receive().await {
                        Ok(res) => {
                            println!("request = {:?}", res);
                            let response = self.respond(res);
                            println!("response = {:?}", response);
                            Server::send(&mut stream, response).await;
                        }
                        Err(e) => {
                            if e.kind() == std::io::ErrorKind::Other {
                                println!("Client disconnected normally");
                            } else {
                                println!("Error occured on stream reading: {:?}", e);
                            }
                            break;
                        }
                    }
                },
            }
        }
    }

    async fn send(stream: &mut SmartSocketStream, response: Response) {
        match stream.send(response).await {
            Ok(()) => {}
            Err(e) => {
                println!("Error occured on send: {:?}", e);
            }
        }
    }

    fn respond(&mut self, res: Request) -> Response {
        match res {
            Request::On => {
                self.status = Status::On;
                Response::Status(self.status.clone())
            }
            Request::Off => {
                self.status = Status::Off;
                Response::Status(self.status.clone())
            }
            Request::Status => Response::Status(self.status.clone()),
            Request::Power => {
                if self.status == Status::On {
                    let mut rng = rand::thread_rng();
                    self.power = 50.0 + rng.gen_range(0.0..1.0);
                } else {
                    self.power = NAN;
                }
                Response::Power(self.power)
            }
        }
    }
}
