use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

use cleverhouse_by_akrutikov::devices::thermometer::udp::Thermometer;

fn main() -> Result<(), Box<dyn Error>> {
    let thermometer = Thermometer::new("UDP Thermometer".to_string());
    thermometer.listen("127.0.0.1:54321".to_string());

    loop {
        println!("temp: {}", thermometer.temperature());
        sleep(Duration::from_millis(500));
    }

    unreachable!();
    Ok(())
}
