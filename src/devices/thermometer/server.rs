use rand::Rng;
use std::net::{ToSocketAddrs, UdpSocket};
use std::thread::sleep;
use std::time::Duration;

pub struct Server {
    socket: UdpSocket,
}

impl Server {
    pub fn bind<A: ToSocketAddrs>(addr: A) -> std::io::Result<Server> {
        let res = UdpSocket::bind(addr)?;
        let server = Server { socket: res };
        Ok(server)
    }

    pub fn run(&self, to: &str) {
        let mut rng = rand::thread_rng();

        loop {
            let temperature: f64 = 25.0 + rng.gen_range(0.0..1.0);
            self.socket
                .send_to(&temperature.to_be_bytes(), to)
                .expect("couldn't send data");
            println!("sent {}", temperature);
            sleep(Duration::from_millis(500));
        }
    }
}
