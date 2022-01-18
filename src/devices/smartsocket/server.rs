use std::{f64::NAN, net::ToSocketAddrs};

use rand::Rng;

use crate::devices::smartsocket::api::{Request, Response, Status};

use super::stp::{SmartSocketServer, SmartSocketStream};

pub struct Server {
    server: SmartSocketServer,
    status: Status,
    power: f64,
}

impl Server {
    pub fn bind<A: ToSocketAddrs>(addr: A) -> std::io::Result<Server> {
        let res = SmartSocketServer::bind(addr)?;
        let server = Server {
            server: res,
            status: Status::Off,
            power: NAN,
        };
        Ok(server)
    }

    pub fn run(&mut self) {
        let mut rng = rand::thread_rng();

        for stream_res in self.server.incoming() {
            match stream_res {
                Err(e) => {
                    println!("Error occured on stream creation: {:?}", e);
                    continue;
                }
                Ok(mut stream) => {
                    let mut receiving: bool = true;
                    while receiving {
                        match stream.receive() {
                            Ok(res) => {
                                println!("request = {:?}", res);
                                match res {
                                    Request::On => {
                                        self.status = Status::On;
                                        Server::send(
                                            &mut stream,
                                            Response::Status(self.status.clone()),
                                        );
                                    }
                                    Request::Off => {
                                        self.status = Status::Off;
                                        Server::send(
                                            &mut stream,
                                            Response::Status(self.status.clone()),
                                        );
                                    }
                                    Request::Status => {
                                        Server::send(
                                            &mut stream,
                                            Response::Status(self.status.clone()),
                                        );
                                    }
                                    Request::Power => {
                                        if self.status == Status::On {
                                            self.power = 50.0 + rng.gen_range(0.0..1.0);
                                        } else {
                                            self.power = NAN;
                                        }
                                        Server::send(&mut stream, Response::Power(self.power));
                                    }
                                }
                            }
                            Err(_) => receiving = false,
                        }
                    }
                }
            }
        }
    }

    pub fn send(stream: &mut SmartSocketStream, response: Response) {
        match stream.send(response) {
            Ok(()) => {}
            Err(e) => {
                println!("Error occured on send: {:?}", e);
            }
        }
    }
}
