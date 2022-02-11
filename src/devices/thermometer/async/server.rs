use rand::Rng;
use std::thread::sleep;
use std::time::Duration;
use tokio::net::{ToSocketAddrs, UdpSocket};

pub struct Server {
    socket: UdpSocket,
}

impl Server {
    pub async fn bind<A: ToSocketAddrs>(addr: A) -> std::io::Result<Server> {
        let res = UdpSocket::bind(addr).await?;
        let server = Server { socket: res };
        Ok(server)
    }

    pub async fn send(&self, to: &str) {
        let temperature: f64 = 25.0 + rand::thread_rng().gen_range(0.0..1.0);
        self.socket
            .send_to(&temperature.to_be_bytes(), to)
            .await
            .expect("couldn't send data");
        println!("sent {}", temperature);
        sleep(Duration::from_millis(50));
    }

    pub async fn run(&self, to: &str) {
        loop {
            self.send(to).await;
        }
    }
}
