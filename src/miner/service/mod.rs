use std::sync::Arc;

use crate::{library::error::AppResult, miner::bootstrap::AppState};

pub mod exchange_rate;
pub mod jwt_service;
pub mod message_queue;
pub mod miner_stat;

#[derive(Clone)]
pub struct Services {
    pub exchange_rate: exchange_rate::Server,
    pub miner_stat: miner_stat::Server,
    pub message_queue: message_queue::Server,
}

impl Services {
    pub fn init() -> Services {
        Services {
            exchange_rate: exchange_rate::Server::init(),
            miner_stat: miner_stat::Server::init(),
            message_queue: message_queue::Server::init(),
        }
    }

    pub async fn serve(&self, app_state: Arc<AppState>) -> AppResult<()> {
        self.exchange_rate.clone().serve(app_state.clone())?;
        self.miner_stat.clone().serve(app_state.clone())?;
        self.message_queue.serve().await?;
        Ok(())
    }

    pub fn shutdown(&self) -> AppResult<()> {
        self.exchange_rate.shutdown()?;
        self.miner_stat.shutdown()?;
        self.message_queue.shutdown()?;
        Ok(())
    }
}
