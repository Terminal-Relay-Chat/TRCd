use axum::{Json, http::StatusCode, response::IntoResponse, routing::{get, post}};
use axum::extract::{Path, State};
use log::{info, warn};
use serde_json::json;
use tokio::sync::broadcast::Sender;

use crate::backend::socket_server::{AppState, ChannelMessage};

#[derive(Debug)]
pub enum ApiError {
    NotFound,
    BadRequest(String),
    InternalServerError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            ApiError::NotFound => (
                StatusCode::NOT_FOUND,
                "404 Not found.".to_string()
            ),
            ApiError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                msg
            ),
            ApiError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server error, which is to say: not your fault. Contact an Admin.".to_string()
            )
        };

        let body = axum::Json(json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}

#[derive(Debug, Clone)]
struct APIState {
    tx: Sender<ChannelMessage>
}

pub struct Server {
    port: usize
}
impl Server {
    
    pub fn new(port: usize) -> Self {
        Server {
            port: port
        }
    }

    pub async fn run(self, tx: Sender<ChannelMessage>) {
        let app = Self::create_app(&self, tx);       
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", self.port)).await.expect("failed to bind server");

        info!("Server Bound on http://0.0.0.0:{}, to see if it is fully up go to http://0.0.0.0:{}/api", self.port, self.port);

        let _instance = axum::serve(listener, app).await.expect("failed to start server.");
    }

    

    fn create_app(&self, tx: Sender<ChannelMessage>) -> axum::Router {
        let state = APIState {
            tx: tx
        };
        axum::Router::new()
            .route("/api", get(Self::health_check))
            .route("/api/messages/{channel_name}", post(Self::new_message))
            .with_state(state)
    }

    async fn health_check() -> impl IntoResponse {
        Json(json!({
            "status": "ok",
            "message": "TRCd Server is running."
        }))
    }
    
    async fn new_message(State(state): State<APIState>, Path(channel_name): Path<String>, body: String) -> Result<(), impl IntoResponse> {
        if body.is_empty() {return Err(ApiError::BadRequest("body length cannot be 0".to_string()))}
        let system_user = crate::socket_server::MessageSender {
                name: String::from("system"),
                handle: String::from("system"),
                provider: String::from("")
        };
        let message = ChannelMessage {
            channel: channel_name, 
            content: body,
            sender: system_user
        };

        let _ = state.tx.send(message);

        Ok(())
    }


}
