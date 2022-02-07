use std::thread::sleep;
use std::time::Duration;

use cleverhouse_by_akrutikov::devices::thermometer::r#async::udp::Thermometer;

#[tokio::main]
async fn main() {
    let thermometer = Thermometer::new("UDP Thermometer".to_string());
    thermometer.listen("127.0.0.1:54321".to_string()).await;
    loop {
        println!("temp: {}", thermometer.temperature());
        sleep(Duration::from_millis(500));
    }
}
