use cleverhouse_by_akrutikov::devices::thermometer::r#async::server::Server;

#[tokio::main]
async fn main() {
    let server = Server::bind("127.0.0.1:12345")
        .await
        .expect("couldn't bind to address");

    server.run("127.0.0.1:54321").await;
}
