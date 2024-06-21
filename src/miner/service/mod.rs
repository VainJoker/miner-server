use std::sync::Arc;

use crate::miner::bootstrap::AppState;

pub mod exchange_rate;
pub mod jwt_service;
pub mod message_queue;
pub mod miner_stat;
pub mod mqtt_service;

#[derive(Clone)]
pub struct Services {
    pub exchange_rate: exchange_rate::Server,
    pub miner_stat: miner_stat::Server,
    pub message_queue: message_queue::Server,
    pub mqtt: mqtt_service::Server,
}

impl Services {
    pub async fn init() -> Services {
        Services {
            exchange_rate: exchange_rate::Server::init().await,
            miner_stat: miner_stat::Server::init().await,
            message_queue: message_queue::Server::init().await,
            mqtt: mqtt_service::Server::init().await,
        }
    }

    pub async fn serve(&self, app_state: Arc<AppState>) {
        self.exchange_rate.clone().serve(app_state.clone()).await;
        self.miner_stat.clone().serve(app_state.clone()).await;
        self.mqtt.clone().serve(app_state.clone()).await;
        self.message_queue.clone().serve(app_state.clone()).await;
    }

    pub async fn shutdown(&self) {
        self.exchange_rate.shutdown().await;
        self.miner_stat.shutdown().await;
        self.message_queue.shutdown().await;
        self.mqtt.shutdown().await;
    }
}

#[allow(async_fn_in_trait)]
pub trait Service {
    async fn init() -> Self;
    async fn serve(&mut self, app_state: Arc<AppState>);
    async fn shutdown(&self);
}
