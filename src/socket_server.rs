//! The Socket server for TRCd is what publishes updates to clients

use std::{error::Error, net::SocketAddr};

use serde::{Serialize, Deserialize};
use axum::{extract::{ConnectInfo, WebSocketUpgrade, ws::{CloseFrame, WebSocket, close_code::UNSUPPORTED}}, response::IntoResponse, routing::any};
use axum::extract::ws::Message;
use log::{info, warn, trace};
use futures_util::{SinkExt, stream::StreamExt};
use std::sync::Arc;

const MAX_STUPID_MESSAGE: u8 = 10; // to prevent useless data abuse

#[derive(Debug, Serialize, PartialEq)]
enum UpdateType {
    MESSAGE,
    SYSTEM, // SYSTEM is for commands or responses to requests from a client
    ERROR,
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
        use tokio::sync::Mutex;

        async fn handle_sock_recv(ws: Arc<Mutex<WebSocket>>, ip: &SocketAddr) -> Result<(), Box<dyn Error>> {
            let mut stupid_message_counter: u8 = 0; // prevent useless message abuse
            loop {
                let message = match ws.lock().await.recv().await {
                    Some(m) => m,
                    None => {return Ok(())}
                };
                let message = match message {
                    Ok(m) => m,
                    Err(e) => {return Err(Box::new(e))},
                };
                match message {
                    Message::Close(_) => {
                        ws.lock().await.send(Message::Close(None)).await?;
                        trace!("client sent close");
                        return Ok(())
                    },
                    Message::Ping(payload) => {
                        ws.lock().await.send(Message::Pong(payload)).await?;
                    },
                    Message::Text(t) => {
                        info!("Message from client: {}", t);
                    },
                    _ => {
                        stupid_message_counter += 1;
                        if stupid_message_counter > MAX_STUPID_MESSAGE {
                            warn!("Weird data exceeded threshold from ip: {}", *ip);
                            ws.lock().await.send(Message::Close(Some(CloseFrame {
                                code: UNSUPPORTED,
                                reason: "This server supports pings and text (utf-8 formatted).".into()
                            }))).await?;

                            return Ok(());
                        }
                    }

                }
            }
        }

        async fn handl_sock_send(ws: Arc<Mutex<WebSocket>>) -> Result<(), Box<dyn std::error::Error>> {
            Ok(())
        }
        
        let ws = Arc::new(Mutex::new(sock));
        tokio::select! {
            _ = handle_sock_recv(ws.clone(), &ip) => {},
            // _ = handl_sock_send(ws.clone()) => {}
        }
        info!("client disconnected")
    }

}
