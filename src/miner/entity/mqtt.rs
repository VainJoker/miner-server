use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Message {
    MessageStatus(MessageStatus),
    MessageUpdate(Box<MessageUpdate>),
}

// impl Message {
//     pub async fn save_into_redis(&self,app_state: Arc<AppState>) ->
// AppResult<()> {         let mut redis = app_state.get_redis().await?;
//         let
//         let mut hasher = DefaultHasher::new();
//         s.hash(&mut hasher);
//         let hash = hasher.finish();
//         let rkey = "mining:status:";
//         redis.set().await?;
//         Ok(())
//     }
// }

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

#[derive(Serialize, Deserialize, Debug)]
struct Coin {
    algorithm: String,
    symbol: String,
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
