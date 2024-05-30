use deadpool_redis::{Connection, Pool, Runtime};
use redis::AsyncCommands;

use crate::library::{
    cfg,
    error::{InnerResult, RedisorError},
};

pub struct Redisor {
    pub pool: Pool,
}

pub struct Redis {
    pub connection: Connection,
}

impl Redisor {
    pub fn init() -> Self {
        let cfg = cfg::config();
        let redis_url = cfg.miner.redis_url.clone();
        let deadpool = deadpool_redis::Config::from_url(redis_url);
        match deadpool.create_pool(Some(Runtime::Tokio1)) {
            Ok(pool) => {
                tracing::info!(
                    "🚀 Connection to the self.connection is successful!"
                );
                Self { pool }
            }
            Err(err) => {
                panic!("💥 Failed to connect to the self.connection: {err:?}");
            }
        }
    }

    pub async fn get_redis(&self) -> InnerResult<Redis> {
        Ok(Redis {
            connection: self
                .pool
                .get()
                .await
                .map_err(RedisorError::PoolError)?,
        })
    }
}

impl Redis {
    pub async fn get(&mut self, key: &str) -> InnerResult<Option<String>> {
        let result: Option<String> = self
            .connection
            .get(key)
            .await
            .map_err(RedisorError::ExeError)?;
        Ok(result)
    }

    pub async fn set(&mut self, key: &str, value: &str) -> InnerResult<()> {
        self.connection
            .set::<_, _, ()>(key, value)
            .await
            .map_err(RedisorError::ExeError)?;
        Ok(())
    }

    pub async fn get_hash_keys(
        &mut self,
        key: &str,
    ) -> InnerResult<Option<Vec<String>>> {
        let result: Option<Vec<String>> = self
            .connection
            .hkeys(key)
            .await
            .map_err(RedisorError::ExeError)?;
        Ok(result)
    }

    pub async fn set_hash(
        &mut self,
        key: &str,
        field: &str,
        value: &str,
    ) -> InnerResult<()> {
        self.connection
            .hset::<_, _, _, ()>(key, field, value)
            .await
            .map_err(RedisorError::ExeError)?;
        Ok(())
    }

    pub async fn del(&mut self, key: &str) -> InnerResult<()> {
        self.connection
            .del::<_, ()>(key)
            .await
            .map_err(RedisorError::ExeError)?;
        Ok(())
    }

    pub async fn set_ex(
        &mut self,
        key: &str,
        value: &str,
        ttl: u64,
    ) -> InnerResult<()> {
        self.connection
            .set_ex::<_, _, ()>(key, value, ttl)
            .await
            .map_err(RedisorError::ExeError)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::time;

    use super::*;

    #[tokio::test]
    async fn test_redisor_init() {
        cfg::init(&"./fixtures/config.toml".to_string());
        let redis_url = cfg::config().miner.redis_url.clone();
        let deadpool = deadpool_redis::Config::from_url(redis_url);
        let pool = deadpool.create_pool(Some(Runtime::Tokio1)).unwrap();
        let redisor = Redisor { pool };
        let mut redis = redisor.get_redis().await.unwrap();

        redis.set("key1", "value").await.unwrap();
        assert_eq!(redis.get("key1").await.unwrap().unwrap(), "value");
        redis.del("key1").await.unwrap();
    }

    #[tokio::test]
    async fn test_redisor_del() {
        cfg::init(&"./fixtures/config.toml".to_string());
        let redis_url = cfg::config().miner.redis_url.clone();
        let deadpool = deadpool_redis::Config::from_url(redis_url);
        let pool = deadpool.create_pool(Some(Runtime::Tokio1)).unwrap();
        let redisor = Redisor { pool };
        let mut redis = redisor.get_redis().await.unwrap();

        redis.set("key2", "value").await.unwrap();
        assert_eq!(redis.get("key2").await.unwrap(), Some("value".to_string()));
        redis.del("key2").await.unwrap();
        assert_eq!(redis.get("key2").await.unwrap(), None);
        redis.del("key2").await.unwrap();
    }

    #[tokio::test]
    #[ignore]
    async fn test_redisor_set_ex() {
        cfg::init(&"./fixtures/config.toml".to_string());
        let redis_url = cfg::config().miner.redis_url.clone();
        let deadpool = deadpool_redis::Config::from_url(redis_url);
        let pool = deadpool.create_pool(Some(Runtime::Tokio1)).unwrap();
        let redisor = Redisor { pool };
        let mut redis = redisor.get_redis().await.unwrap();
        redis.del("key3").await.unwrap();
        redis.set_ex("key3", "value", 10).await.unwrap();
        assert_eq!(redis.get("key3").await.unwrap(), Some("value".to_string()));
        tokio::time::sleep(time::Duration::from_millis(10000)).await;
        assert_eq!(redis.get("key3").await.unwrap(), None);
    }

    #[tokio::test]
    async fn test_redisor_set_hash() {
        cfg::init(&"./fixtures/config.toml".to_string());
        let redis_url = cfg::config().miner.redis_url.clone();
        let deadpool = deadpool_redis::Config::from_url(redis_url);
        let pool = deadpool.create_pool(Some(Runtime::Tokio1)).unwrap();
        let redisor = Redisor { pool };
        let mut redis = redisor.get_redis().await.unwrap();
        redis.del("key4").await.unwrap();
        redis.set_hash("key4", "field1", "value1").await.unwrap();
    }

    #[tokio::test]
    async fn test_redisor_get_hash_keys() {
        cfg::init(&"./fixtures/config.toml".to_string());
        let redis_url = cfg::config().miner.redis_url.clone();
        let deadpool = deadpool_redis::Config::from_url(redis_url);
        let pool = deadpool.create_pool(Some(Runtime::Tokio1)).unwrap();
        let redisor = Redisor { pool };
        let mut redis = redisor.get_redis().await.unwrap();
        redis.del("key5").await.unwrap();
        assert_eq!(redis.get_hash_keys("key5").await.unwrap(), Some(vec![]));
        redis.set_hash("key5", "field1", "value1").await.unwrap();
        redis.set_hash("key5", "field2", "value2").await.unwrap();
        assert_eq!(
            redis.get_hash_keys("key5").await.unwrap(),
            Some(vec!["field1".to_string(), "field2".to_string()])
        );
    }
}
