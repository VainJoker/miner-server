use std::sync::Arc;

use sqlx::types::Json;
use tonic::{Request, Response, Status};

use super::bootstrap::{shutdown_signal, AppState};
use crate::{
    library::{cfg, error::AppResult},
    models::{
        account_setting::BwAccountSetting,
        machine::{BwMachine, CreateBwMachineSchema, Setting},
    },
    pb::{
        self,
        miner_sign::{
            miner_sign_server::{MinerSign, MinerSignServer},
            SignRequest, SignResponse,
        },
    },
};

pub struct Server {
    pub host: &'static str,
    pub port: usize,
    pub app_state: Arc<AppState>,
}

#[tonic::async_trait]
impl MinerSign for Server {
    async fn sign(
        &self,
        request: Request<SignRequest>,
    ) -> Result<Response<SignResponse>, Status> {
        let inner = request.into_inner();
        // println!("{:#?}", inner);
        self.store(inner).await.expect("Failed");

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
        Self {
            host,
            port,
            app_state,
        }
    }

    pub async fn serve(self) {
        let addr = format!("{}:{}", self.host, self.port);
        let addr = addr.parse().unwrap_or_else(|e| {
            panic!("💥 Failed to connect bind TcpListener: {e:?}")
        });
        let signer = MinerSignServer::new(self);

        tracing::info!("✨ listening on {}", addr);

        tonic::transport::Server::builder()
            .trace_fn(|_| tracing::info_span!("grpc_server"))
            .add_service(signer)
            .serve_with_shutdown(addr, shutdown_signal())
            .await
            .unwrap_or_else(|e| panic!("💥 Failed to start ABI server: {e:?}"));
    }

    pub async fn store(&self, sign: SignRequest) -> AppResult<()> {
        let mut redis = self.app_state.get_redis().await?;

        let r_user_key = format!("m_user:{}", sign.key);
        let account_id = match redis.get(&r_user_key).await? {
            Some(i) => i,
            None => {
                let i = BwAccountSetting::fetch_account_id_by_key(
                    self.app_state.get_db(),
                    &sign.key,
                )
                .await?;
                redis.set_ex(&r_user_key, i, 259200).await?;
                i
            }
        };

        let cap = sign.capability.expect("Failed");
        let power_modes = pb::get_energy_modes(cap.powermode);
        let crypto_coin = pb::get_coins(cap.algoset);

        let item = CreateBwMachineSchema {
            mac: &sign.mac,
            account_id,
            device_type: &sign.devtype,
            device_name: "",
            device_ip: &sign.ip,
            setting: Json(Setting {
                crypto_coin,
                power_modes,
                pool_maximal: cap.poolmax as usize,
                support_boot: cap.reboot == 1,
                support_reset: cap.reset == 1,
                support_update: cap.update == 1,
                support_led: cap.led == 1,
            }),
            hardware_version: &sign.hv,
            software_version: &sign.sv,
        };
        BwMachine::create_bw_machine(self.app_state.get_db(), &item)
            .await
            .expect("Failed to add machine");
        Ok(())
    }
}
