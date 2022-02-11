use serde::de::DeserializeOwned;
use serde::Serialize;

use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

pub struct SmartSocketServer {
    listener: TcpListener,
}

pub struct SmartSocketStream {
    pub stream: TcpStream,
}

impl SmartSocketServer {
    pub async fn bind<A: ToSocketAddrs>(addr: A) -> std::io::Result<Self> {
        let listener = TcpListener::bind(addr).await?;
        let server = SmartSocketServer { listener };
        Ok(server)
    }

    pub async fn accept(&self) -> std::io::Result<SmartSocketStream> {
        let (stream, _) = self.listener.accept().await?;
        let connection = SmartSocketStream { stream };
        Ok(connection)
    }
}

impl SmartSocketStream {
    pub async fn connect<A: ToSocketAddrs>(addr: A) -> std::io::Result<SmartSocketStream> {
        let res = TcpStream::connect(addr).await?;
        let stream = SmartSocketStream { stream: res };
        Ok(stream)
    }

    pub async fn receive<T: DeserializeOwned>(&self) -> std::io::Result<T> {
        let mut buf = [0; 1024];
        let size = self.read(&mut buf).await?;
        if size == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "End of stream",
            ));
        }
        println!("read = {:?}", size);
        let slice = &buf[0..size];
        let res: T = serde_json::from_slice(slice)?;
        Ok(res)
    }

    async fn read(&self, buf: &mut [u8]) -> std::io::Result<usize> {
        loop {
            match self.stream.try_read(buf) {
                Ok(res) => return Ok(res),
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::WouldBlock {
                        continue;
                    } else {
                        return Err(e);
                    }
                }
            }
        }
    }

    pub async fn send<T: Serialize>(&mut self, msg: T) -> std::io::Result<()> {
        self.stream.writable().await?;
        let str = serde_json::to_string(&msg)?;
        println!("msg = {:?}", str);
        println!("written = {:?}", self.stream.try_write(str.as_bytes())?);
        Ok(())
    }
}
