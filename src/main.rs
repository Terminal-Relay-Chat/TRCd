use tokio;
use log::{warn, info};
mod server;
mod socket_server;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    
    let server = server::Server::new(3000);
    let socks = socket_server::SocketServer::new(3001);
    tokio::select! {
        _ = server.run() => { warn!("server crashed!") },
        _ = socks.run() => { warn!("socket server crashed!") },
    }
}
