//! The Socket server for TRCd is what publishes updates to clients

use std::net::SocketAddr;

use serde::{Serialize, Deserialize};
use axum::{extract::{ConnectInfo, WebSocketUpgrade, ws::WebSocket}, response::IntoResponse, routing::any};
use axum::extract::ws::Message;
use log::{info, warn};
use futures_util::{SinkExt, stream::StreamExt};

#[derive(Debug, Serialize, PartialEq)]
enum UpdateType {
    MESSAGE,
    SYSTEM, // SYSTEM is for commands or responses to requests from a client
    ERROR
}

#[derive(Serialize, Debug)]
struct SocketMessage {
    message_type: UpdateType,
    content: String
}

pub struct SocketServer {
    port: usize,
}
impl SocketServer {
    pub fn new(port: usize) -> Self {
        SocketServer {
            port: port 
        }
    } 
    
    fn create_app() -> axum::Router {
       axum::Router::new()
           .route("/", any(Self::ws_handler))
    }

    pub async fn run(self) {
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", self.port))
            .await
            .expect("unable to bind websocket");
        let app = Self::create_app();
        info!("Socket server bound to ws://0.0.0.0:{}", self.port);
        axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.expect("Error starting the websocket service")
    }

    async fn ws_handler(ws: WebSocketUpgrade, ConnectInfo(address): ConnectInfo<SocketAddr>) -> impl IntoResponse {
        ws.on_upgrade(move |socket| Self::handle_socket(socket, address))
    }

    async fn handle_socket(mut sock: WebSocket, ip: SocketAddr) {
        loop {
            sock.send(Message::text("Hello World!")).await;
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    }


}
