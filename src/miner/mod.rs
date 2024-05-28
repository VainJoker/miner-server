pub mod api;
pub mod bootstrap;
pub mod entity;
pub mod service;

use std::sync::Arc;

use tokio::net::TcpListener;

use crate::{
    library::cfg,
    miner::{
        api::route,
        bootstrap::{shutdown_signal, AppState},
    },
};

pub async fn serve() {
    let cfg = cfg::config();
    let miner_state = Arc::new(AppState::init().await);
    // Create a regular axum app.
    let app = route::init(miner_state.clone());

    // Create a `TcpListener` using tokio.
    let listener =
        TcpListener::bind(format!("{}:{}", &cfg.miner.host, &cfg.miner.port))
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



    // Run the server with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(miner_state.clone()))
        .await
        .unwrap_or_else(|e| panic!("💥 Failed to start webserver: {e:?}"));
}


// pub struct Server {
//     api: Arc<Mutex<api::Server>>,
//     mqtt: Arc<Mutex<mqtt::Server>>,
//     cancel: Option<oneshot::Sender<()>>,
// }
//
// impl Server {
//     pub fn new() -> Result<Server, Box<dyn std::error::Error>> {
//         // Initialize MySQL and Redis clients here
//         let db = Arc::new(Mutex::new(mysqlx::default_client()?));
//         let redis = Arc::new(Mutex::new(redisx::default_client()?));
//
//         instance::set_db(db.clone());
//         instance::set_redis(redis.clone());
//
//         let api = Arc::new(Mutex::new(api::new()));
//         let mqtt = Arc::new(Mutex::new(mqtt::new(/* mqtt options here */)));
//
//         Ok(Server {
//             api,
//             mqtt,
//             cancel: None,
//         })
//     }
//
//     pub async fn run(&mut self) {
//         let (cancel_tx, cancel_rx) = oneshot::channel();
//         self.cancel = Some(cancel_tx);
//
//         let api_future = {
//             let api = self.api.clone();
//             tokio::spawn(async move {
//                 api.lock().unwrap().run(cancel_rx).await;
//             })
//         };
//
//         let mqtt_future = {
//             let mqtt = self.mqtt.clone();
//             tokio::spawn(async move {
//                 mqtt.lock().unwrap().run(cancel_rx).await;
//             })
//         };
//
//         let _ = tokio::try_join!(api_future, mqtt_future);
//     }
//
//     pub fn shutdown(&mut self) {
//         if let Some(cancel) = self.cancel.take() {
//             let _ = cancel.send(());
//         }
//
//         self.api.lock().unwrap().shutdown();
//         self.mqtt.lock().unwrap().shutdown();
//     }
// }