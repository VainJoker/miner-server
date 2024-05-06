use deadpool_redis::{Connection, Pool, Runtime};

use crate::library::{
    cfg,
    error::{InnerResult, RedisorError},
};

pub type Redis = Connection;
pub struct Redisor {
    pub pool: Pool,
}

impl Redisor {
    pub fn init() -> Self {
        let cfg = cfg::config();
        let redis_url = cfg.inpay.redis_url.clone();
        let deadpool = deadpool_redis::Config::from_url(redis_url);
        match deadpool.create_pool(Some(Runtime::Tokio1)) {
            Ok(pool) => {
                tracing::info!("🚀 Connection to the redis is successful!");
                Self { pool }
            }
            Err(err) => {
                panic!("💥 Failed to connect to the redis: {err:?}");
            }
        }
    }

    pub async fn get_conn(&self) -> InnerResult<Redis> {
        Ok(self.pool.get().await.map_err(RedisorError::PoolError)?)
    }
}
