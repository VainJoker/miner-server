use std::sync::Arc;

use tonic::{Request, Response, Status};

use super::bootstrap::{shutdown_signal, AppState};
use crate::{
    library::cfg,
    pb::miner_sign::{
        miner_sign_server::{MinerSign, MinerSignServer},
        SignRequest, SignResponse,
    },
};

pub struct Server {
    pub host: &'static str,
    pub port: usize,
    pub app_state: Arc<AppState>
}

#[tonic::async_trait]
impl MinerSign for Server {
    async fn sign(
        &self,
        request: Request<SignRequest>,
    ) -> Result<Response<SignResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = SignResponse {
            result: 0,
            ms: "".to_string(),
            mpt: 0,
            mu: "".to_string(),
            mp: "".to_string(),
            t: 0,
        };
        Ok(Response::new(reply))
    }
}

impl Server {
    pub fn init(app_state: Arc<AppState>) -> Self {
        let config = cfg::config();
        let host = &config.miner.grpc_host;
        let port = config.miner.grpc_port;
        Self { host, port, app_state }
    }

    pub async fn serve(self) {
        let addr = format!("{}:{}", self.host, self.port);
        let addr = addr.parse().unwrap_or_else(|e| {
            panic!("💥 Failed to connect bind TcpListener: {e:?}")
        });
        let signer = MinerSignServer::new(self);

        tracing::info!(
            "✨ listening on {}", addr
        );

        tonic::transport::Server::builder()
            .trace_fn(|_| tracing::info_span!("grpc_server"))
            .add_service(signer)
            .serve_with_shutdown(addr, shutdown_signal())
            .await
            .unwrap_or_else(|e| panic!("💥 Failed to start ABI server: {e:?}"));

    }
}
