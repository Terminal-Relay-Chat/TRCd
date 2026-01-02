use tokio;
use log::{warn, info};
use tokio::sync::mpsc::unbounded_channel;

#[forbid(unsafe_code)]

mod backend;
mod authentication;
mod database;

use backend::socket_server;

use crate::database::database::DBCalls; // backend::server serves the api and is instantiated by the socket
                            // server

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {serve().await}
    // otherwise the bin is being run to manipulate entries
    
    new_user().await;
}

async fn new_user() {
    use crate::database::sqlite::db_sqlite::{DB_Sqlite, DB_DEFAULT_URL};

    let connection = DB_Sqlite::new(DB_DEFAULT_URL).await;
    connection.setup().await;

}

async fn serve() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    
    let socks = socket_server::SocketServer::new(3001);
    // bizzarly, I have to start the api from the socket server because of some shared state. It's
    // weird. (see create_app()). This is to share a broadcast tx instance to the API so that users
    // can send new messages in a more "secure" fassion.
    socks.run().await;
}
