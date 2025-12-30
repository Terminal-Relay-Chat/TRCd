use tokio;
use log::{warn, info};
use tokio::sync::mpsc::unbounded_channel;
mod server;
mod socket_server;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    
    let socks = socket_server::SocketServer::new(3001);
    // bizzarly, I have to start the api from the socket server because of some shared state. It's
    // weird. (see create_app()). This is to share a broadcast tx instance to the API so that users
    // can send new messages in a more "secure" fassion.
    socks.run().await;
}
