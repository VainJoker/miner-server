use tonic::{Request, Response, Status};
use crate::library::cfg;
use crate::library::error::AppResult;
use crate::pb::miner_sign::miner_sign_server::{MinerSign, MinerSignServer};
use crate::pb::miner_sign::{SignRequest, SignResponse};

pub struct Server{
    pub host: String,
    pub port: usize,
}

#[tonic::async_trait]
impl MinerSign for Server {
    async fn sign(&self, request: Request<SignRequest>) -> Result<Response<SignResponse>, Status> {
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
    pub fn init(host: &str, port: usize) -> Self {
        Self {
            host: host.to_string(),
            port
        }
    }

    pub async fn serve(&self) -> AppResult<()>{
        let addr = format!("{}:{}", self.host, self.port);
        let addr = addr.parse().expect("gRPC server address should be a valid socket address");
        let signer = MinerSignServer::new(self);

        Ok(tonic::transport::Server::builder()
            .add_service(signer)
            .serve(addr)
            .await?)
    }


}