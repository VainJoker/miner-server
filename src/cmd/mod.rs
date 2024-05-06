use crate::{
    library::{cfg, logger},
    miner,
};

pub async fn web_serve(){
    tracing::info!("Application started");
    cfg::init(&"./fixtures/config.toml".to_string());
    let (_guard1,_guard2,_guard3) = logger::init(cfg::config());

    miner::serve().await;
    tracing::info!("Application stopped");
}