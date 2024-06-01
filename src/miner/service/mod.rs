use std::sync::Arc;

use crate::miner::bootstrap::AppState;

pub mod exchange_rate;
pub mod jwt_service;
pub mod message_queue;
pub mod miner_stat;
pub mod mqtt;

#[derive(Clone)]
pub struct Services {
    pub exchange_rate: exchange_rate::Server,
    pub miner_stat: miner_stat::Server,
    pub message_queue: message_queue::Server,
    pub mqtt: mqtt::Server,
}

impl Services {
    pub async fn init() -> Services {
        Services {
            exchange_rate: exchange_rate::Server::init(),
            miner_stat: miner_stat::Server::init(),
            message_queue: message_queue::Server::init(),
            mqtt: mqtt::Server::init().await,
        }
    }

    pub async fn serve(&self, app_state: Arc<AppState>) {
        self.exchange_rate.clone().serve(app_state.clone());
        self.miner_stat.clone().serve(app_state.clone());
        self.mqtt.clone().serve(app_state.clone()).await;
        self.message_queue.serve().await;
    }

    pub async fn shutdown(&self) {
        self.exchange_rate.shutdown();
        self.miner_stat.shutdown();
        self.message_queue.shutdown();
        self.mqtt.shutdown().await;
    }
}
