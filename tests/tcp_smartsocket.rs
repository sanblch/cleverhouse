use std::{
    thread::{self, sleep},
    time::Duration,
};

use cleverhouse_by_akrutikov::devices::smartsocket::{
    server::Server, stp::SmartSocketStream, tcp::SmartSocket,
};

#[test]
fn run_test() {
    thread::spawn(|| {
        let mut server = Server::bind("127.0.0.1:12345").unwrap();
        server.run();
    });

    sleep(Duration::from_millis(100));
    let client = SmartSocketStream::connect("127.0.0.1:12345").unwrap();
    let mut smartsocket = SmartSocket::new(client, String::from("a smart socket"));
    assert_eq!(smartsocket.is_on(), false);
    assert_eq!(smartsocket.power().is_nan(), true);
    smartsocket.on();
    let p1 = smartsocket.power();
    assert_eq!(smartsocket.is_on(), true);
    assert_eq!(p1 > 50.0 && p1 < 51.0, true);
    smartsocket.off();
    assert_eq!(smartsocket.is_on(), false);
    assert_eq!(smartsocket.power().is_nan(), true);
    smartsocket.on();
    assert_eq!(smartsocket.is_on(), true);
    smartsocket.shutdown();
}
