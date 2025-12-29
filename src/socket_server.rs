//! The Socket server for TRCd is what publishes updates to clients

use std::{error::Error, net::SocketAddr};

use serde::{Serialize, Deserialize};
use axum::{extract::{ConnectInfo, State, WebSocketUpgrade, ws::{CloseFrame, WebSocket, close_code::UNSUPPORTED}}, response::IntoResponse, routing::any};
use axum::extract::ws::Message;
use log::{info, warn, trace};
use std::sync::Arc;
use tokio::sync::broadcast::{self, Receiver};

const MAX_STUPID_MESSAGE: u8 = 10; // to prevent useless data abuse

#[derive(Debug, Serialize, PartialEq)]
enum UpdateType {
    MESSAGE,
    SYSTEM, // SYSTEM is for commands or responses to requests from a client
    ERROR,
}

enum UserActiveChannel {
    String(String),
    None,
    All
}

#[derive(Serialize, Debug)]
struct SocketMessage {
    message_type: UpdateType,
    content: String
}

#[derive(Debug, Clone)]
pub struct ChannelMessage {
    channel: String,
    content: String,
    sender: MessageSender 
}

#[derive(Debug, Clone)]
pub struct MessageSender {
    name: String,
    handle: String,
    provider: String
}


#[derive(Debug, Clone)]
pub struct AppState {
    tx: broadcast::Sender<ChannelMessage>,
}

pub struct SocketServer {
    port: usize,
}
impl SocketServer {
    pub fn new(port: usize) -> Self {
        SocketServer {
            port: port,
        }
    } 
    
    fn create_app() -> axum::Router {
        let (tx, _) = broadcast::channel::<ChannelMessage>(1024);
        let state = AppState { 
            tx: tx
        };

        axum::Router::new()
            .route("/", any(Self::ws_handler))
            .with_state(state)

    }

    pub async fn run(self) {
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", self.port))
            .await
            .expect("unable to bind websocket");
        let app = Self::create_app();
        info!("Socket server bound to ws://0.0.0.0:{}", self.port);

        axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.expect("Error starting the websocket service")
    }

    async fn ws_handler(ws: WebSocketUpgrade, ConnectInfo(address): ConnectInfo<SocketAddr>, State(state): State<AppState>) -> impl IntoResponse {
        ws.on_upgrade(move |socket| Self::handle_socket(socket, address, State(state)))
    }

    async fn handle_socket(mut sock: WebSocket, ip: SocketAddr, State(state): State<AppState>) {
        use tokio::sync::Mutex;
        let rx = state.tx.subscribe();
        let tx = Arc::new(state.tx);

        async fn handle_sock_recv(ws: Arc<Mutex<WebSocket>>, ip: &SocketAddr, tx: Arc<broadcast::Sender<ChannelMessage>>) -> Result<(), Box<dyn Error>> {
            let mut stupid_message_counter: u8 = 0; // prevent useless message abuse
            let mut active_channel = UserActiveChannel::None;

            let system_user = MessageSender {
                name: String::from("system"),
                handle: String::from("system"),
                provider: String::from("")
            };


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
                        tx.send(ChannelMessage {
                            channel: "general".into(),
                            content: t.to_string(),
                            sender: system_user.clone() 
                        })?;
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

        async fn handle_sock_send(ws: Arc<Mutex<WebSocket>>, mut rx: Receiver<ChannelMessage>) -> Result<(), Box<dyn std::error::Error>> {
            loop {
                let message = rx.recv().await;
                match message {
                    Ok(m) => {
                        ws.lock().await.send(Message::Text(format!("New message: {}", m.content).into())).await?;
                    },
                    Err(_) => {
                        warn!("caught a channel recv error. Closing connection to reduce load.");
                        return Err("caught a channel recv error".into());
                    }
                }
            }
        }

        
        let ws = Arc::new(Mutex::new(sock));
        tokio::select! {
            _ = handle_sock_recv(ws.clone(), &ip, tx) => {},
            _ = handle_sock_send(ws.clone(), rx) => {}
        }
        info!("client disconnected")
    }

}
