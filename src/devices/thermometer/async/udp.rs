use crate::device::Device;
use std::{
    f64::NAN,
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};
use tokio::net::UdpSocket;

pub struct Thermometer {
    pub description: String,
    temperature: Arc<Mutex<f64>>,
}

impl Device for Thermometer {
    fn description(&self) -> &str {
        &*self.description
    }
}

impl Thermometer {
    pub fn new(description: String) -> Self {
        Thermometer {
            description,
            temperature: Arc::new(Mutex::new(NAN as f64)),
        }
    }

    pub async fn listen(&self, from: String) {
        let temperature_ref = self.temperature.clone();
        let socket = UdpSocket::bind(from)
            .await
            .expect("couldn't bind to address");
        let mut buf = [0; 8];
        tokio::spawn(async move {
            loop {
                let (number_of_bytes, _) = socket
                    .recv_from(&mut buf)
                    .await
                    .expect("Didn't receive data");
                if number_of_bytes > 0 {
                    let mut lock = temperature_ref.lock().unwrap();
                    *lock = f64::from_be_bytes(buf);
                }
                sleep(Duration::from_millis(50));
            }
        });
    }

    pub fn temperature(&self) -> f64 {
        *self.temperature.lock().unwrap()
    }
}
