use std::{
    thread::{self, sleep},
    time::Duration,
};

use cleverhouse_by_akrutikov::devices::thermometer::{server::Server, udp::Thermometer};

#[test]
fn run_test() {
    thread::spawn(|| {
        let server = Server::bind("127.0.0.1:12345").expect("couldn't bind to address");
        server.run("127.0.0.1:54321");
    });

    let thermometer = Thermometer::new("UDP Thermometer".to_string());
    thermometer.listen("127.0.0.1:54321".to_string());

    sleep(Duration::from_millis(100));
    let t1 = thermometer.temperature();
    sleep(Duration::from_millis(100));
    let t2 = thermometer.temperature();
    sleep(Duration::from_millis(100));
    let t3 = thermometer.temperature();
    assert_eq!(t1 != t2 && t2 != t3, true);
    assert_eq!(!t1.is_nan() && !t2.is_nan() && !t3.is_nan(), true);
    assert_eq!(
        t1 > 25.0 && t2 > 25.0 && t3 > 25.0 && t1 < 26.0 && t2 < 26.0 && t3 < 26.0,
        true
    );
}
