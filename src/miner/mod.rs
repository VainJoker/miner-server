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
        service::mq_customer,
    },
};

// TODO:
// 我需要使用一个Server 决定所有 serevice 是否应该停止
// 这是一个示例
// use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
// use std::thread;
//
// struct Server {
//     api: ApiServer,
//     mqtt: MqttServer,
//     should_stop: Arc<AtomicBool>,
// }
//
// impl Server {
//     fn new() -> Self {
//         Server {
//             api: ApiServer::new(),
//             mqtt: MqttServer::new(),
//             should_stop: Arc::new(AtomicBool::new(false)),
//         }
//     }
//
//     fn run(&self) {
//         let api_should_stop = Arc::clone(&self.should_stop);
//         let mqtt_should_stop = Arc::clone(&self.should_stop);
//
//         let api_thread = thread::spawn(move || {
//             self.api.run(api_should_stop);
//         });
//
//         let mqtt_thread = thread::spawn(move || {
//             self.mqtt.run(mqtt_should_stop);
//         });
//
//         api_thread.join().unwrap();
//         mqtt_thread.join().unwrap();
//     }
//
//     fn shutdown(&self) {
//         self.should_stop.store(true, Ordering::SeqCst);
//     }
// }
//
// struct ApiServer;
//
// impl ApiServer {
//     fn new() -> Self {
//         ApiServer
//     }
//
//     fn run(&self, should_stop: Arc<AtomicBool>) {
//         while !should_stop.load(Ordering::SeqCst) {
//             // Run the API server here
//         }
//     }
// }
//
// struct MqttServer;
//
// impl MqttServer {
//     fn new() -> Self {
//         MqttServer
//     }
//
//     fn run(&self, should_stop: Arc<AtomicBool>) {
//         while !should_stop.load(Ordering::SeqCst) {
//             // Run the MQTT server here
//         }
//     }
// }
//
// fn main() {
//     let server = Server::new();
//     server.run();
//     server.shutdown();
// }

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

    // Run the MQCustomer
    tokio::spawn(mq_customer::MqCustomer::serve(miner_state.clone()));

    // Run the server with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(miner_state))
        .await
        .unwrap_or_else(|e| panic!("💥 Failed to start webserver: {e:?}"));
}
