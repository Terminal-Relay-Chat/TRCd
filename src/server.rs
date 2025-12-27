use axum::routing::{get, post};
use log::{info, warn};

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
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", self.port)).await.unwrap();
        info!("Server Bound on http://0.0.0.0:{}, to see if it is fully up go to http://0.0.0.0:{}/api", self.port, self.port);
        let _instance = axum::serve(listener, app).await.unwrap();
    }

    fn create_app() -> axum::Router {
        axum::Router::new()
            .route("/api", get(Self::health_check))
    }

    async fn health_check() -> &'static str {
        "TRCd Server is up"
    }

}
