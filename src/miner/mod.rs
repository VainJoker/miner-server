pub mod abi;
pub mod api;
pub mod bootstrap;
pub mod entity;
pub mod service;

use std::sync::Arc;

use crate::miner::bootstrap::AppState;

pub async fn serve() {
    let miner_state = Arc::new(AppState::init().await);

    AppState::serve(miner_state.clone()).await;

    let miner_state1 = miner_state.clone();
    let api_server = tokio::spawn(async move {
        api::Server::init(miner_state1).serve().await;
    });

    let miner_state2 = miner_state.clone();
    let abi_server = tokio::spawn(async move {
        abi::Server::init(miner_state2).serve().await;
    });

    let _ = tokio::join!(api_server, abi_server);

    miner_state.services.shutdown().await;
}
