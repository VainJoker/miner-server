use std::sync::Arc;

use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    library::{error::AppResult, Redis},
    miner::bootstrap::AppState,
    models::account_setting::BwAccountSetting,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Message {
    MessageStatus(MessageStatus),
    MessageUpdate(Box<MessageUpdate>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageStatus {
    mode: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageUpdate {
    #[serde(rename = "nowrate")]
    now_rate: f64,
    #[serde(rename = "avgrate")]
    avg_rate: f64,
    #[serde(rename = "historyrate")]
    history_rate: Vec<f64>,
    #[serde(rename = "powermode")]
    power_mode: String,
    #[serde(rename = "digtime")]
    dig_time: i32,
    pool: Vec<Pool>,
    #[serde(rename = "harderr")]
    hard_err: f64,
    refuse: f64,
    temperature: String,
    fan: String,
    led: i32,
    ip: String,
    key: String,
    #[serde(rename = "coin", deserialize_with = "from_coin")]
    coin: Option<Coin>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageUpdateBasic {
    pool: Vec<Pool>,
    led: i32,
    ip: String,
    #[serde(rename = "coin", deserialize_with = "from_coin")]
    coin: Option<Coin>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Pool {
    url: String,
    user: String,
    legal: bool,
    active: bool,
    #[serde(rename = "dragid")]
    drag_id: i32,
    #[serde(rename = "pool-priority")]
    pool_priority: i32,
    pass: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Coin {
    algorithm: String,
    symbol: String,
}

impl From<&MessageUpdate> for MessageUpdateBasic {
    fn from(value: &MessageUpdate) -> Self {
        MessageUpdateBasic {
            pool: value.pool.clone(),
            led: value.led,
            ip: value.ip.clone(),
            coin: value.coin.clone(),
        }
    }
}

impl Message {
    pub async fn store(
        &self,
        app_state: Arc<AppState>,
        mac: &str,
    ) -> AppResult<()> {
        match self {
            Message::MessageStatus(status) => {
                status.store(&mut app_state.get_redis().await?, mac).await
            }
            Message::MessageUpdate(update) => {
                update.store(mac, app_state).await
            }
        }
    }
}

impl MessageStatus {
    pub async fn store(&self, redis: &mut Redis, mac: &str) -> AppResult<()> {
        let r_status_key = format!("miner_status:{}", mac);
        let r_status_value = serde_json::to_string(self).map_err(|e| {
            let format_err =
                format!("Error occurred while serializing message: {}", e);
            tracing::error!("{}", format_err);
            anyhow::anyhow!(format_err)
        })?;
        redis.hset(&r_status_key, "mode", &r_status_value).await?;
        redis.expire(&r_status_key, 60 * 60).await?;
        tracing::debug!("Updated miner status for MAC: {}", mac);
        Ok(())
    }
}

impl MessageUpdate {
    pub async fn store(
        &self,
        mac: &str,
        app_state: Arc<AppState>,
    ) -> AppResult<()> {
        let mut redis = app_state.get_redis().await?;
        let r_status_key = format!("miner_status:{}", mac);
        let r_status_value = serde_json::to_string(&self).map_err(|e| {
            let format_err =
                format!("Error occurred while serializing message: {}", e);
            tracing::error!("{}", format_err);
            anyhow::anyhow!(format_err)
        })?;
        redis.hset(&r_status_key, "status", &r_status_value).await?;
        redis.expire(&r_status_key, 60).await?;
        tracing::debug!("Updated miner update for MAC: {}", mac);

        let r_account_key = format!("account_key:{}", self.key);
        let account_id = match redis.get(&r_account_key).await? {
            Some(id) => id,
            None => {
                let account_id = BwAccountSetting::fetch_account_id_by_key(
                    app_state.get_db(),
                    &self.key,
                )
                .await?
                .to_string();
                redis.set(&r_account_key, &account_id).await?;
                account_id
            }
        };
        let r_user_key = format!("miner_user:{}", account_id);
        let basic_value = MessageUpdateBasic::from(self);
        let r_user_value =
            serde_json::to_string(&basic_value).map_err(|e| {
                let format_err =
                    format!("Error occurred while serializing message: {}", e);
                tracing::error!("{}", format_err);
                anyhow::anyhow!(format_err)
            })?;
        redis.hset(&r_user_key, mac, &r_user_value).await?;
        redis.expire(&r_user_key, 3600 * 24 * 30).await?;
        tracing::debug!("Updated user key for account ID: {}", account_id);

        Ok(())
    }
}

fn from_coin<'de, D>(deserializer: D) -> Result<Option<Coin>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    Ok(match s.as_str() {
        "blake2b(SC)" => Some(Coin {
            algorithm: "blake2b".to_string(),
            symbol: "SC".to_string(),
        }),
        "eaglesong(CKB)" => Some(Coin {
            algorithm: "eaglesong".to_string(),
            symbol: "CKB".to_string(),
        }),
        "blake3(ALPH)" => Some(Coin {
            algorithm: "blake3".to_string(),
            symbol: "ALPH".to_string(),
        }),
        "blake2s(KDA)" => Some(Coin {
            algorithm: "blake2s".to_string(),
            symbol: "KDA".to_string(),
        }),
        "scrypt(LTC)" => Some(Coin {
            algorithm: "scrypt".to_string(),
            symbol: "LTC".to_string(),
        }),
        "cnr(STC)" => Some(Coin {
            algorithm: "cnr".to_string(),
            symbol: "STC".to_string(),
        }),
        "lbry(LBC)" => Some(Coin {
            algorithm: "lbry".to_string(),
            symbol: "LBC".to_string(),
        }),
        "blake2bsha3(HNS)" => Some(Coin {
            algorithm: "blake2bsha3".to_string(),
            symbol: "HNS".to_string(),
        }),
        "kHeavyHash(kaspa)" => Some(Coin {
            algorithm: "kHeavyHash".to_string(),
            symbol: "KAS".to_string(),
        }),
        "kHeavyHash(KAS)" => Some(Coin {
            algorithm: "kHeavyHash".to_string(),
            symbol: "KAS".to_string(),
        }),
        &_ => None,
    })
}

// TODO: move into tests
// #[cfg(test)]
// mod tests {
//     use tokio::test;
//     use crate::library::cfg;

// #[tokio::test]
// async fn test_message_status_store() {
//     cfg::init(&"./fixtures/config.toml".to_string());
//     let app_state = Arc::new(AppState::init().await);
//     let mut redis = app_state.get_redis().await.unwrap();
//     redis.expect_set_hash()
//         .with(eq("miner_status:mac_address"), eq("mode"), any())
//         .times(1)
//         .returning(|_, _, _| Ok(()));
//     redis.expect_expire()
//         .with(eq("miner_status:mac_address"), eq(60))
//         .times(1)
//         .returning(|_, _| Ok(()));
//
//     let app_state = Arc::new(MockAppState {
//         redis: redis,
//     });
//
//     let message_status = MessageStatus { mode: 1 };
//     let result = message_status.store(&mut
// app_state.get_redis().await.unwrap(), "mac_address").await;
//
//     assert!(result.is_ok());
// }
//
// #[tokio::test]
// async fn test_message_update_store() {
//     cfg::init(&"./fixtures/config.toml".to_string());
//     let app_state = Arc::new(AppState::init().await);
//     let mut redis = app_state.get_redis().await.unwrap();
//     redis.expect_set_hash()
//         .with(eq("miner_status:mac_address"), eq("status"), any())
//         .times(1)
//         .returning(|_, _, _| Ok(()));
//     redis.expect_expire()
//         .with(eq("miner_status:mac_address"), eq(60))
//         .times(1)
//         .returning(|_, _| Ok(()));
//     redis.expect_get()
//         .with(eq("account_key:key"))
//         .times(1)
//         .returning(|_| Ok(Some("account_id".to_string())));
//     redis.expect_set_hash()
//         .with(eq("miner_user:account_id"), eq("mac_address"), any())
//         .times(1)
//         .returning(|_, _, _| Ok(()));
//     redis.expect_expire()
//         .with(eq("miner_user:account_id"), eq(3600 * 24 * 30))
//         .times(1)
//         .returning(|_, _| Ok(()));
//
//     let app_state = Arc::new(MockAppState {
//         redis: redis,
//     });
//
//     let message_update = Box::new(MessageUpdate {
//         now_rate: 1.0,
//         avg_rate: 1.0,
//         history_rate: vec![1.0],
//         power_mode: "mode".to_string(),
//         dig_time: 1,
//         pool: vec![],
//         hard_err: 0.0,
//         refuse: 0.0,
//         temperature: "temp".to_string(),
//         fan: "fan".to_string(),
//         led: 1,
//         ip: "127.0.0.1".to_string(),
//         key: "key".to_string(),
//         coin: None,
//     });
//
//     let result = message_update.store("mac_address",
// app_state.clone()).await;
//
//     assert!(result.is_ok());
// }
// }
