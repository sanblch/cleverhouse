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
                                let response =
                                    Server::respond(res, &mut self.status, &mut self.power);
                                Server::send(&mut stream, response);
                            }
                            Err(_) => receiving = false,
                        }
                    }
                }
            }
        }
    }

    fn send(stream: &mut SmartSocketStream, response: Response) {
        match stream.send(response) {
            Ok(()) => {}
            Err(e) => {
                println!("Error occured on send: {:?}", e);
            }
        }
    }

    fn respond(res: Request, status: &mut Status, power: &mut f64) -> Response {
        match res {
            Request::On => {
                *status = Status::On;
                Response::Status(status.clone())
            }
            Request::Off => {
                *status = Status::Off;
                Response::Status(status.clone())
            }
            Request::Status => Response::Status(status.clone()),
            Request::Power => {
                if *status == Status::On {
                    let mut rng = rand::thread_rng();
                    *power = 50.0 + rng.gen_range(0.0..1.0);
                } else {
                    *power = NAN;
                }
                Response::Power(*power)
            }
        }
    }
}
