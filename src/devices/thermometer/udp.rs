use crate::device::Device;
use std::{
    f64::NAN,
    net::UdpSocket,
    sync::{Arc, Mutex},
    thread::{sleep, spawn},
    time::Duration,
};

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

    pub fn listen(&self, from: String) {
        let temperature_ref = self.temperature.clone();
        spawn(move || {
            let socket = UdpSocket::bind(from).expect("couldn't bind to address");
            let mut buf = [0; 8];
            loop {
                let (number_of_bytes, _) = socket.recv_from(&mut buf).expect("Didn't receive data");
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
