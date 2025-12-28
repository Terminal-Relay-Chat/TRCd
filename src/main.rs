use tokio;
mod server;
mod socket_server;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    let server = server::Server::new(3000);
    //TODO: implement the socket_server
    server.run().await;
}
