use std::sync::{Arc, Mutex};
use tokio::signal;
use tokio::sync::oneshot;
use tower::util::Either::A;
use crate::library::cfg;
use crate::library::error::AppResult;
use crate::miner::bootstrap::AppState;

pub mod exchange_rate;
pub mod miner_stat;
pub mod mq_customer;


pub struct Server<'a>{
    pub exchange_rate: Arc<Mutex<exchange_rate::Server<'a>>>,
    pub miner_stat: Arc<Mutex<miner_stat::Server<'a>>>,
    pub mq_customer: Arc<Mutex<mq_customer::Server>>,
    // pub cancel: Option<oneshot::Sender<()>>,
}

impl Server<'_> {
    pub fn new() -> Server<'static> {
        Server {
            exchange_rate: Arc::new(Mutex::new(exchange_rate::Server::init())),
            miner_stat: Arc::new(Mutex::new(miner_stat::Server::init())),
            mq_customer: Arc::new(Mutex::new(mq_customer::Server::init())),
            // cancel: None,
        }
    }

    pub async fn run(&self) {
        self.exchange_rate.lock().unwrap().serve().await.unwrap();
        self.miner_stat.lock().unwrap().serve().await.unwrap();
        self.mq_customer.lock().unwrap().serve().await.unwrap();
    }

    pub fn shutdown(&self) -> AppResult<()> {
        // self.exchange_rate.lock().unwrap().shutdown().unwrap();
        // self.miner_stat.lock().unwrap().shutdown().unwrap();
        self.mq_customer.lock().unwrap().shutdown().unwrap();
        Ok(())
    }

    pub async fn shutdown_signal(&self) {
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
                self.shutdown().unwrap();
                tracing::info!("Ctrl+C signal received.");
        },
        () = terminate => {
                self.shutdown().unwrap();
                tracing::info!("Terminate signal received.");
        },
    }
    }
}


