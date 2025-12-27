use tokio;
mod server;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    let server = server::Server::new(3000);
    server.run().await;
}
