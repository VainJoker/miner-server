use std::sync::Arc;
use tokio::net::TcpListener;
use crate::library::cfg;
use crate::library::error::AppResult;
use crate::miner::bootstrap::{AppState, shutdown_signal};

pub mod controller;
pub mod middleware;
pub mod route;


pub struct Server{
    pub host: String,
    pub port: String,

}

impl Server {
    pub fn init(host: String, port: String) -> Self {
        Self {
            host,
            port
        }
    }

    pub async fn serve(&self,miner_state: Arc<AppState>) {

        // let miner_state = Arc::new(AppState::init().await);
        // Create a regular axum app.
        let app = route::init(miner_state.clone());

        // let cfg = cfg::config();
        let listener =
            TcpListener::bind(format!("{}:{}", self.host, self.port))
                .await
                .unwrap_or_else(|e| {
                    panic!("💥 Failed to connect bind TcpListener: {e:?}")
                });

        tracing::info!(
        "✨ listening on {}",
            listener.local_addr().unwrap_or_else(|e| panic!(
                "💥 Failed to connect bind TcpListener: {e:?}"
            ))
        );

        // miner_state.clone().serve().await;

        // Run the server with graceful shutdown
        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal(miner_state.clone()))
            .await
            .unwrap_or_else(|e| panic!("💥 Failed to start webserver: {e:?}"));
    }
}