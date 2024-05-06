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

pub async fn serve() {
    let cfg = cfg::config();
    let inpay_state = Arc::new(AppState::init().await);
    // Create a regular axum app.
    let app = route::init(inpay_state.clone());

    // Create a `TcpListener` using tokio.
    let listener =
        TcpListener::bind(format!("{}:{}", &cfg.inpay.host, &cfg.inpay.port))
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
    tokio::spawn(mq_customer::MqCustomer::serve(inpay_state.clone()));

    // Run the server with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(inpay_state))
        .await
        .unwrap_or_else(|e| panic!("💥 Failed to start webserver: {e:?}"));
}
