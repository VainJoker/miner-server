use std::sync::Arc;

use chrono::Utc;
use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    library::error::AppResult, miner::bootstrap::AppState,
    models::machine::Coin,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Message {
    MessageMode(MessageMode),
    MessageStatus(Box<MessageStatus>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageMode {
    pub mode: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageStatus {
    #[serde(rename = "nowrate")]
    pub now_rate: f64,
    #[serde(rename = "avgrate")]
    pub avg_rate: f64,
    #[serde(rename = "historyrate")]
    pub(crate) history_rate: Vec<f64>,
    #[serde(rename = "powermode")]
    pub(crate) power_mode: String,
    #[serde(rename = "digtime")]
    pub(crate) dig_time: i32,
    pub(crate) pool: Vec<Pool>,
    #[serde(rename = "harderr")]
    pub(crate) hard_err: f64,
    pub(crate) refuse: f64,
    pub(crate) temperature: String,
    pub(crate) fan: String,
    pub(crate) led: i32,
    pub(crate) ip: String,
    key: String,
    #[serde(rename = "coin", deserialize_with = "from_coin")]
    pub(crate) coin: Option<Coin>,
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct MessageUpdateBasic {
//     pub(crate) pool: Vec<Pool>,
//     // led: i32,
//     pub(crate) ip: String,
//     #[serde(rename = "coin", deserialize_with = "from_coin")]
//     pub(crate) coin: Option<Coin>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pool {
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

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum CoinFormat {
    Simple(String),
    Detailed(Coin),
}

// impl From<&MessageUpdate> for MessageUpdateBasic {
//     fn from(value: &MessageUpdate) -> Self {
//         MessageUpdateBasic {
//             pool: value.pool.clone(),
//             ip: value.ip.clone(),
//             coin: value.coin.clone(),
//         }
//     }
// }

// TODO: Check?
impl Message {
    pub async fn store(
        &self,
        app_state: Arc<AppState>,
        mac: &str,
    ) -> AppResult<()> {
        let mut redis = app_state.get_redis().await?;
        let r_key = format!("miner_status:{}", mac);
        let r_field = match self {
            Message::MessageMode(_) => "mode",
            Message::MessageStatus(_) => "status",
        };
        let r_value = serde_json::to_string(&self).map_err(|e| {
            let format_err =
                format!("Error occurred while serializing message: {}", e);
            tracing::error!("{}", format_err);
            anyhow::anyhow!(format_err)
        })?;
        redis.hset(&r_key, r_field, &r_value).await?;
        let t_now = Utc::now().timestamp();
        redis.hset(&r_key, "time", &t_now).await?;
        redis.expire(&r_key, 259200).await?;
        Ok(())
    }
}

fn from_coin<'de, D>(deserializer: D) -> Result<Option<Coin>, D::Error>
where
    D: Deserializer<'de>,
{
    let coin_format = CoinFormat::deserialize(deserializer)?;
    match coin_format {
        CoinFormat::Detailed(d) => Ok(Some(d)),
        CoinFormat::Simple(s) => Ok(match s.as_str() {
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
        }),
    }
}

// TODO: move into tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_coin() {
        let coin = Coin {
            algorithm: "blake2b".to_string(),
            symbol: "SC".to_string(),
        };
        let json = serde_json::to_string(&coin).unwrap();
        assert_eq!(json, r#"{"algorithm":"blake2b","symbol":"SC"}"#);
    }
}

// #[test]
// fn test_deserialize_message() {
// {"nowrate":220.692,"avgrate":210.25,"historyrate":[216.759,205.234,
// 207.718,227.19,213.357,208.49700000000004,225.146,215.259,200.357,
// 210.665,226.502,223.918,206.987,206.52599999999995,207.542,211.443,
// 217.706,214.922,198.74200000000005,205.44299999999996,202.125,197.
// 71200000000005,214.31,203.07,191.48,214.429,220.8,237.94099999999992,
// 217.15400000000008,228.028,207.86000000000004,205.56,210.539,215.239,
// 201.03299999999996,205.002,229.43400000000008,217.49,218.593,200.359,
// 210.262,221.445,208.655,204.15400000000005,199.252,214.253,208.
// 55900000000003,202.75099999999995,219.104,211.51,210.228,196.
// 18700000000004,199.43099999999995,212.485,208.74400000000003,215.789,
// 206.98,194.602,226.64,237.462,214.545,199.75599999999997,222.671,203.
// 95,204.168,214.7,224.52,225.678,238.774,227.976,198.18799999999996,
// 200.90599999999995,221.554,201.24900000000005,216.951,196.898,186.
// 727,223.518,216.12,213.865,202.498,193.208,207.081,220.146,210.347,
// 224.34,232.418,209.47900000000004,199.605,207.709,211.987,207.
// 56200000000004,221.45,210.75099999999992,207.46099999999996,206.514,
// 196.697,201.60299999999995,184.502,208.28,211.963,201.82,217.742,216.
// 014,206.66099999999997,218.612,211.317,219.583,215.622,202.584,199.
// 05599999999995,202.00799999999995,220.327,209.36000000000004,231.382,
// 220.773,235.612,202.87400000000005,234.212,213.366,213.032,189.
// 87200000000004,210.18,216.919,213.293,202.03099999999995,201.
// 07400000000004,206.24,217.094,215.932,196.03299999999996,203.162,189.
// 30900000000003,185.718,214.018,214.25,220.201,226.638,229.302,207.
// 261,213.565,222.299,222.156,207.055,214.113,206.24599999999995,221.
// 01,205.525,214.341,201.07299999999995,192.60400000000004,203.
// 00299999999996,202.37799999999996,208.71900000000005,210.085,231.908,
// 211.142,202.59599999999995,216.491,197.79499999999996,209.12,204.
// 96099999999996,202.86700000000005,220.258,223.724,195.29499999999996,
// 197.174,216.609,197.31799999999996,187.433,215.394,235.96,229.265,
// 196.06,221.818,210.062,210.00599999999991,206.91400000000004,217.205,
// 208.638,220.345,215.05,235.841,185.528,211.757,197.79,209.107,218.
// 965,215.86900000000009,210.368,197.274,205.19099999999997,211.457,
// 228.044,197.77900000000005,209.12400000000005,192.696,230.418,204.5,
// 199.11000000000004,202.707,202.606,208.56799999999996,200.
// 24099999999996,224.29,210.41,206.487,198.40900000000005,210.645,215.
// 47099999999992,192.82400000000004,198.377,212.493,200.857,201.53,218.
// 297,218.94,213.901,219.535,202.19,220.894,216.00599999999991,212.507,
// 203.12,205.59400000000005,211.231,226.861,218.27900000000008,199.
// 55099999999996,191.78299999999996,218.982,219.448,200.34400000000005,
// 216.356,203.386,214.811,211.90400000000008,201.50799999999995,221.
// 03599999999992,210.418,218.882,205.138,218.547,211.122,212.549,243.
// 66099999999992,191.766,199.65900000000005,206.636,203.882,217.02,218.
// 397,207.56299999999996,208.435,212.27900000000008,212.78599999999992,
// 219.135,210.156,216.651,224.93400000000008,214.893,195.761,197.053,
// 213.854,219.43,213.015,223.581,204.81900000000005,198.44299999999996,
// 220.55900000000008,212.595,221.862,212.985,223.44099999999992,212.
// 826,210.018,205.554,200.745,194.14599999999996,208.15099999999995,
// 218.886,209.236,227.052,210.841,211.336,202.81,206.67700000000005,
// 221.995],"powermode":"Hashrate","digtime":1314,"pool":[{"url":"
// stratum+tcp://192.168.111.225:4001","user":"shminer.3134","legal":
// true,"active":true,"dragid":0,"pool-priority":0,"pass":"123"}],"
// harderr":0.998976,"refuse":0.0017722640673460345,"temperature":"78.6
// °C","fan":"1980 /
// 1920","led":0,"ip":"192.168.110.97","key":"
// dd99c54c08bb2752f5f8ad6e526243cbea722","coin":{"algorithm":"scrypt","
// symbol":"LTC"}} "{\n \"historyrate\": [\n  218.965,\n
// 215.86900000000006,\n  210.368,\n  197.274,\n  205.19099999999997,\n
// 211.457,\n  228.044,\n  197.77900000000003,\n  209.12400000000003,\n
// 192.696,\n  230.418,\n  204.5,\n  199.11000000000003,\n  202.707,\n
// 202.606,\n  208.56799999999997,\n  200.24099999999997,\n  224.29,\n
// 210.41,\n  206.487,\n  198.40900000000003,\n  210.645,\n
// 215.47099999999994,\n  192.82400000000003,\n  198.377,\n  212.493,\n
// 200.857,\n  201.53,\n  218.297,\n  218.94,\n  213.901,\n  219.535,\n
// 202.19,\n  220.894,\n  216.00599999999994,\n  212.507,\n  203.12,\n
// 205.59400000000003,\n  211.231,\n  226.861,\n  218.27900000000006,\n
// 199.55099999999997,\n  191.78299999999997,\n  218.982,\n  219.448,\n
// 200.34400000000003,\n  216.356,\n  203.386,\n  214.811,\n
// 211.90400000000006,\n  201.50799999999997,\n  221.03599999999994,\n
// 210.418,\n  218.882,\n  205.138,\n  218.547,\n  211.122,\n
// 212.549,\n  243.66099999999994,\n  191.766,\n  199.65900000000003,\n
// 206.636,\n  203.882,\n  217.02,\n  218.397,\n  207.56299999999997,\n
// 208.435,\n  212.27900000000006,\n  212.78599999999994,\n  219.135,\n
// 210.156,\n  216.651,\n  224.93400000000006,\n  214.893,\n  195.761,\n
// 197.053,\n  213.854,\n  219.43,\n  213.015,\n  223.581,\n
// 204.81900000000003,\n  198.44299999999997,\n  220.55900000000006,\n
// 212.595,\n  221.862,\n  212.985,\n  223.44099999999994,\n  212.826,\n
// 210.018,\n  205.554,\n  200.745,\n  194.14599999999997,\n
// 208.15099999999997,\n  218.886,\n  209.236,\n  227.052,\n  210.841,\n
// 211.336,\n  202.81,\n  206.67700000000003,\n  221.995,\n  228.713,\n
// 236.877,\n  221.88,\n  210.823,\n  217.75099999999994,\n  213.541,\n
// 188.919,\n  202.4,\n  201.171,\n  205.00299999999997,\n  200.195,\n
// 186.31099999999997,\n  195.97299999999997,\n  212.854,\n  175.513,\n
// 195.918,\n  207.81900000000003,\n  212.007,\n  214.06,\n
// 202.72299999999997,\n  207.21900000000003,\n  209.728,\n  227.195,\n
// 220.051,\n  193.09799999999997,\n  190.3,\n  220.87400000000006,\n
// 205.79,\n  232.486,\n  227.563,\n  212.768,\n  213.421,\n  210.052,\n
// 204.37,\n  206.475,\n  204.138,\n  219.137,\n  223.621,\n  211.73,\n
// 203.05599999999997,\n  192.155,\n  205.333,\n  189.32299999999997,\n
// 188.19299999999997,\n  216.107,\n  189.18700000000003,\n
// 189.22299999999997,\n  210.761,\n  208.942,\n  195.03,\n  209.919,\n
// 211.76,\n  219.454,\n  214.179,\n  198.11000000000003,\n  220.181,\n
// 214.334,\n  220.893,\n  215.883,\n  197.226,\n  203.7,\n  201.234,\n
// 189.016,\n  194.951,\n  207.18,\n  223.764,\n  193.52200000000003,\n
// 213.034,\n  231.894,\n  215.81,\n  200.77900000000003,\n  192.78,\n
// 189.593,\n  215.957,\n  213.247,\n  200.155,\n  206.62799999999997,\n
// 188.97400000000003,\n  194.72400000000003,\n  224.89,\n  220.13,\n
// 194.671,\n  228.138,\n  226.37,\n  223.734,\n  231.054,\n
// 197.18599999999997,\n  195.46099999999997,\n  207.861,\n  210.143,\n
// 217.035,\n  225.497,\n  210.796,\n  207.25900000000003,\n
// 206.53799999999997,\n  222.114,\n  214.472,\n  218.854,\n
// 190.97299999999997,\n  212.482,\n  203.01,\n  204.328,\n  217.073,\n
// 221.852,\n  208.36599999999997,\n  189.56299999999997,\n  211.174,\n
// 206.988,\n  212.531,\n  214.145,\n  231.149,\n  212.84,\n  212.182,\n
// 200.278,\n  196.91,\n  201.08000000000003,\n  208.18400000000003,\n
// 202.11000000000003,\n  208.58700000000003,\n  223.938,\n  226.351,\n
// 231.363,\n  217.681,\n  210.576,\n  223.89,\n  204.50900000000003,\n
// 214.014,\n  202.02200000000003,\n  184.102,\n  199.52099999999997,\n
// 222.283,\n  211.29,\n  199.43,\n  241.148,\n  200.287,\n  204.565,\n
// 217.746,\n  213.299,\n  205.37400000000003,\n  215.547,\n
// 215.06599999999994,\n  216.918,\n  199.757,\n  209.673,\n  214.785,\n
// 221.93,\n  211.192,\n  219.926,\n  229.761,\n  227.853,\n  218.423,\n
// 237.627,\n  214.805,\n  201.082,\n  206.99099999999997,\n
// 207.06299999999997,\n  202.97400000000003,\n  214.794,\n  201.91,\n
// 230.483,\n  224.867,\n  201.16099999999997,\n  209.102,\n  220.07,\n
// 213.259,\n  214.91099999999994,\n  220.891,\n  223.926,\n  225.841,\n
// 202.1,\n  196.593,\n  198.769,\n  212.28599999999994,\n  220.742,\n
// 208.81599999999997,\n  216.223,\n  214.88,\n  217.678,\n  193.541,\n
// 209.49900000000003,\n  226.5,\n  211.483,\n  223.247,\n  220.282,\n
// 199.63400000000003,\n  220.628,\n  206.35299999999997\n ],\n \"ip\":
// \"192.168.110.97\",\n \"digtime\": 1502,\n \"powermode\":
// \"Hashrate\",\n \"temperature\": \"77.1 \xc2\xb0C\",\n \"refuse\":
// 0.0026964560862865949,\n \"harderr\": 0.998983,\n \"led\": 0,\n
// \"avgrate\": 210.146,\n \"fan\": \"1980 / 1980\",\n \"nowrate\":
// 232.477,\n \"pool\": [\n  {\n   \"url\":
// \"stratum+tcp://192.168.111.225:4001\",\n   \"legal\": true,\n
// \"active\": true,\n   \"dragid\": 0,\n   \"user\":
// \"shminer.3134\",\n   \"pool-priority\": 0,\n   \"pass\": \"123\"\n
// }\n ],\n \"coin\": \"scrypt(LTC)\",\n \"key\":
// \"dd99c54c08bb2752f5f8ad6e526243cbea722\"\n}"
//     }
// }
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

//     let app_state = Arc::new(MockAppState {
//         redis: redis,
//     });

//     let message_status = MessageStatus { mode: 1 };
//     let result = message_status.store(&mut
// app_state.get_redis().await.unwrap(), "mac_address").await;

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
//         .returning(|_| Ok(Some("uid".to_string())));
//     redis.expect_set_hash()
//         .with(eq("miner_user:uid"), eq("mac_address"), any())
//         .times(1)
//         .returning(|_, _, _| Ok(()));
//     redis.expect_expire()
//         .with(eq("miner_user:uid"), eq(3600 * 24 * 30))
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
