use axum::{Json, http::StatusCode, response::IntoResponse, routing::{get, post}};
use log::{info, warn};
use serde_json::json;

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

pub struct Server {
    port: usize
}
impl Server {
    
    pub fn new(port: usize) -> Self {
        Server {
            port: port
        }
    }

    pub async fn run(self) {
        let app = Self::create_app();       
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", self.port)).await.expect("failed to bind server");
        info!("Server Bound on http://0.0.0.0:{}, to see if it is fully up go to http://0.0.0.0:{}/api", self.port, self.port);
        let _instance = axum::serve(listener, app).await.expect("failed to start server.");
    }

    fn create_app() -> axum::Router {
        axum::Router::new()
            .route("/api", get(Self::health_check))
    }

    async fn health_check() -> impl IntoResponse {
        Json(json!({
            "status": "ok",
            "message": "TRCd Server is running."
        }))
    }


}
