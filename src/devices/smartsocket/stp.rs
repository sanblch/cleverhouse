use serde::de::DeserializeOwned;
use serde::Serialize;

use std::io::{Read, Write};
use std::net::{Incoming, Shutdown, TcpListener, TcpStream, ToSocketAddrs};

pub struct SmartSocketServer {
    listener: TcpListener,
}

pub struct SmartSocketStream {
    pub stream: TcpStream,
}

pub struct SmartSocketIncoming<'a> {
    pub incoming: Incoming<'a>,
}

impl SmartSocketServer {
    pub fn bind<A: ToSocketAddrs>(addr: A) -> std::io::Result<Self> {
        let listener = TcpListener::bind(addr)?;
        let server = SmartSocketServer { listener };
        Ok(server)
    }

    pub fn incoming(&self) -> SmartSocketIncoming<'_> {
        let incoming = self.listener.incoming();
        SmartSocketIncoming { incoming }
    }
}

impl Iterator for SmartSocketIncoming<'_> {
    fn next(&mut self) -> Option<Self::Item> {
        let res = self.incoming.next()?;
        match res {
            Ok(e) => {
                let stream = SmartSocketStream { stream: e };
                Some(Ok(stream))
            }
            Err(e) => Some(Err(e)),
        }
    }

    type Item = std::io::Result<SmartSocketStream>;
}

impl SmartSocketStream {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> std::io::Result<SmartSocketStream> {
        let res = TcpStream::connect(addr)?;
        let stream = SmartSocketStream { stream: res };
        Ok(stream)
    }

    pub fn receive<T: DeserializeOwned>(&mut self) -> std::io::Result<T> {
        let mut buf = [0; 1024];
        let size = self.stream.read(&mut buf)?;
        let slice = &buf[0..size];
        let res: T = serde_json::from_slice(slice)?;
        Ok(res)
    }

    pub fn send<T: Serialize>(&mut self, msg: T) -> std::io::Result<()> {
        let str = serde_json::to_string(&msg)?;
        self.stream.write_all(str.as_bytes())?;
        Ok(())
    }

    pub fn shutdown(&self) -> std::io::Result<()> {
        self.stream.shutdown(Shutdown::Both)
    }
}
