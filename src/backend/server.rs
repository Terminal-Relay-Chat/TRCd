//! File containing the API backend 

use axum::{
    Json, body::Body, extract::{Extension, Path, State}, http::{HeaderMap, Request, StatusCode}, middleware::Next, response::{IntoResponse, Response}, routing::{get, post}
};
use log::{info, warn};
use serde::{Serialize, Deserialize};
use serde_json::json;
use tokio::sync::broadcast::Sender;
use std::sync::Arc;
use crate::{authentication::middleware::authenticate, database::database::DBCalls};
use crate::database::sqlite::db_sqlite::{DB_Sqlite, DB_DEFAULT_URL};
use crate::backend::socket_server::{ChannelMessage};


#[derive(Debug)]
pub enum ApiError {
    NotFound,
    BadRequest(String),
    InternalServerError,
    Unauthorized
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
            ),
            ApiError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "either missing a token (x-auth-token) or an invalid token.".to_string()
            )
        };

        let body = axum::Json(json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct APIResponse {
    error: bool,
    value: String
}
impl APIResponse {
    pub fn serialize(&self) -> String {
        match serde_json::to_string(&self) {
            Ok(value) => value,
            Err(e) => {
                warn!("Serialization Error: {}", e);
                "{\"error\": true, \"value\": \"error serializing response, this is very bizzare. Contact an admin.\"}".to_string()
            }
        }
    }

    pub fn new(error: bool, value: &str) -> Self {
        APIResponse { error: error, value: value.to_string() }
    }
}

#[derive(Debug, Clone)]
pub struct APIState {
    pub tx: Sender<ChannelMessage>,
    pub db: DB_Sqlite
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
        let app = Self::create_app(&self, tx).await;       
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", self.port)).await.expect("failed to bind server");

        info!("Server Bound on http://0.0.0.0:{}, to see if it is fully up go to http://0.0.0.0:{}/api", self.port, self.port);

        let _instance = axum::serve(listener, app).await.expect("failed to start server.");
    }

    

    async fn create_app(&self, tx: Sender<ChannelMessage>) -> axum::Router {
        
        let db_conn = DB_Sqlite::new(DB_DEFAULT_URL).await;
        db_conn.setup().await;

        let state = APIState {
            tx: tx,
            db: db_conn
        };
        axum::Router::new()
            .route("/api/login", post(crate::authentication::routes::login)) // if I remember right, browsers hate when get requests
            .route("/api/messages/{channel_name}", post(Self::new_message))
            .route("/api", get(Self::health_check))
            .with_state(state)
    }

    async fn health_check() -> impl IntoResponse {
        Json(json!({
            "status": "ok",
            "message": "TRCd Server is running."
        }))
    }
    
    async fn new_message(State(state): State<APIState>, Path(channel_name): Path<String>, headers: HeaderMap, body: String) -> Result<&'static str, impl IntoResponse> {
        // authenticate the user
        let user = match authenticate(headers).await {
            Ok(user) => user,
            Err(e) => return Err(ApiError::Unauthorized)
        };

        if body.is_empty() {return Err(ApiError::BadRequest("body length cannot be 0".to_string()))}

        let message = ChannelMessage {
            channel: channel_name, 
            content: body,
            sender: user
        };

        let _ = state.tx.send(message);

        Ok("{\"error\": false}")
    }

}
