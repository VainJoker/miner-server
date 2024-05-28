pub mod constants;

use std::sync::Arc;

use tokio::signal;

use crate::library::{dber::DB, error::AppResult, Dber, Mqer, Redis, Redisor};
use crate::miner::service::Services;

pub struct AppState {
    pub db: Dber,
    pub redis: Redisor,
    pub services: Services<'static>
}

impl AppState {
    pub async fn init() -> Self {
        Self {
            db: Dber::init().await,
            redis: Redisor::init(),
            services: Services::init(),
        }
    }

    pub async fn serve(&self) {
        match self.services.serve().await{
            Ok(_) => (),
            Err(e) => {
                tracing::error!("Failed to start services: {}", e);
                std::process::exit(1);
            }
        }
    }
    pub const fn get_db(&self) -> &DB {
        &self.db.pool
    }

    pub async fn get_redis(&self) -> AppResult<Redis> {
        Ok(self.redis.get_conn().await?)
    }

    pub fn get_mq(&self) -> AppResult<Mqer> {
        Ok(self.services.message_queue.mqer.clone())
    }

}

pub async fn shutdown_signal(app_state: Arc<AppState>) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {
            tracing::info!("Ctrl+C signal received.");
        },
        () = terminate => {
            tracing::info!("Terminate signal received.");
        },
    }
    match app_state.services.shutdown(){
        Ok(_) => (),
        Err(e) => {
            tracing::error!("Failed to shutdown services: {}", e);
            std::process::exit(1);
        }
    }
}
