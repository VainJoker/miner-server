use serde_derive::{Deserialize, Serialize};
use sqlx::types::Json;

use crate::{
    miner::entity::mqtt::Pool,
    models::machine::{Coin, Setting},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReadMachineResponse {
    mac: String,
    uid: i64,

    mode: usize,
    now_rate: Option<f64>,
    avg_rate: Option<f64>,
    history_rate: Option<Vec<f64>>,
    power_mode: Option<String>,
    dig_time: Option<i32>,
    pool: Vec<Pool>,
    hard_err: Option<f64>,
    refuse: Option<f64>,
    temperature: Option<String>,
    fan: Option<String>,
    led: Option<i32>,

    coin: Option<Coin>,
    device_type: String,
    device_name: String,
    device_ip: String,

    group_id: Option<i64>,
    group_name: Option<String>,
    policy_id: Option<i64>,
    policy_name: Option<String>,

    pool_id: Option<i64>,
    pool_name: Option<String>,

    setting: Json<Setting>,

    hardware_version: String,
    software_version: String,
}

impl ReadMachineResponse {
    // pub fn from_online(
    //     config: &BwMachine,
    //     status: MessageUpdate,
    //     mode: MessageStatus,
    // ) -> Self {
    //     Self {
    //         mac: config.mac.clone(),
    //         uid: config.uid,
    //         mode: mode.mode,
    //         now_rate: Some(status.now_rate),
    //         avg_rate: Some(status.avg_rate),
    //         history_rate: Some(status.history_rate),
    //         power_mode: Some(status.power_mode),
    //         dig_time: Some(status.dig_time),
    //         pool: status.pool,
    //         hard_err: Some(status.hard_err),
    //         refuse: Some(status.refuse),
    //         temperature: Some(status.temperature),
    //         fan: Some(status.fan),
    //         led: Some(status.led),
    //         coin: status.coin,
    //         device_type: config.device_type.clone(),
    //         device_name: config.device_name.clone(),
    //         device_ip: status.ip,
    //         group_id: config.group_id,
    //         group_name: Some("".to_string()),
    //         policy_id: config.policy_id,
    //         policy_name: Some("".to_string()),
    //         pool_id: config.pool_id,
    //         pool_name: Some("".to_string()),
    //         setting: config.setting.clone(),
    //         hardware_version: config.hardware_version.clone(),
    //         software_version: config.software_version.clone(),
    //     }
    // }

    // pub fn from_offline(
    //     config: &BwMachine,
    //     status: MessageUpdateBasic,
    // ) -> Self {
    //     Self {
    //         mac: config.mac.clone(),
    //         uid: config.uid,
    //         mode: 0,
    //         now_rate: None,
    //         avg_rate: None,
    //         history_rate: None,
    //         power_mode: None,
    //         dig_time: None,
    //         pool: status.pool,
    //         hard_err: None,
    //         refuse: None,
    //         temperature: None,
    //         fan: None,
    //         led: None,
    //         coin: status.coin,
    //         device_type: config.device_type.clone(),
    //         device_name: config.device_name.clone(),
    //         device_ip: status.ip,
    //         group_id: config.group_id,
    //         group_name: Some("".to_string()),
    //         policy_id: config.policy_id,
    //         policy_name: Some("".to_string()),
    //         pool_id: config.pool_id,
    //         pool_name: Some("".to_string()),
    //         setting: config.setting.clone(),
    //         hardware_version: config.hardware_version.clone(),
    //         software_version: config.software_version.clone(),
    //     }
    // }
}

// pub async fn get_machines(
//     app_state: Arc<AppState>,
//     uid: i64,
// ) -> AppResult<()> {
//     let mut redis = app_state.get_redis().await?;
//     let r_user_key = &format!("miner_user:{}", uid);

//     let r_user_fields = redis.hkeys(r_user_key).await?.unwrap();

//     let r_status_keys: Vec<_> = r_user_fields
//         .iter()
//         .map(|k| format!("miner_status:{}", k))
//         .collect();
//     let r_status_keys: Vec<_> =
//         r_status_keys.iter().map(|s| s.as_str()).collect();

//     let r_status_value = redis.hgetalls(&r_status_keys).await?;
//     eprintln!("000{:#?}", r_status_value);
//     let mut online_map: HashMap<String, (MessageUpdate, MessageStatus)> =
//         HashMap::new();
//     let mut none_indices = Vec::new();
//     for (i, v) in r_status_value.iter().enumerate() {
//         if v.is_empty() {
//             none_indices.push(i);
//             continue;
//         }
//         let status_str = v.get("status").unwrap();
//         let mode_str = v.get("mode").unwrap();
//         eprintln!("111:{:#?}", status_str);
//         eprintln!("222:{:#?}", mode_str);
//         let online_status: MessageUpdate =
//             serde_json::from_str(status_str).unwrap();
//         let online_mode: MessageStatus =
//             serde_json::from_str(mode_str).unwrap();
//         online_map.insert(
//             r_user_fields.get(i).unwrap().clone().to_lowercase(),
//             (online_status, online_mode),
//         );
//     }
//     eprintln!("333{:#?}", online_map);
//     eprintln!("444{:#?}", none_indices);

//     let mut offline_map: HashMap<String, MessageUpdateBasic> =
// HashMap::new();     if !none_indices.is_empty() {
//         let none_machine_hash_keys: Vec<_> = none_indices
//             .into_iter()
//             .map(|i| &r_user_fields[i])
//             .collect();
//         let none_machine_hash_keys: Vec<_> =
//             none_machine_hash_keys.iter().map(|s| s.as_str()).collect();
//         eprintln!("555{:#?}", none_machine_hash_keys);
//         let r_user_values =
//             redis.hgets(r_user_key, &none_machine_hash_keys).await?;
//         eprintln!("666{:#?}", r_user_values);

//         for (i, v) in r_user_values.iter().enumerate() {
//             let status_str = v.clone().unwrap();
//             let offline_status: MessageUpdateBasic =
//                 serde_json::from_str(&status_str).unwrap();
//             offline_map.insert(
//                 r_user_fields.get(i).unwrap().clone().to_lowercase(),
//                 offline_status,
//             );
//         }
//     }

//     let bw_machines =
//         BwMachine::fetch_machines_by_uid(app_state.get_db(),
// uid)             .await
//             .expect("Failed to fetch machines");
//     let mut db_hash: HashMap<String, BwMachine> = HashMap::new();
//     for bw_machine in bw_machines {
//         db_hash.insert(bw_machine.mac.clone().to_lowercase(), bw_machine);
//     }

//     eprintln!("888{:#?}", db_hash);

//     let mut machines = Vec::new();
//     for (mac, (status, mode)) in online_map {
//         let config = db_hash.get(&mac).unwrap();

//         let machine = ReadMachineResponse::from_online(config, status, mode);
//         eprintln!("{:#?}", machine);
//         machines.push(machine);
//     }

//     for (mac, status) in offline_map {
//         let config = db_hash.get(&mac).unwrap();
//         let machine = ReadMachineResponse::from_offline(config, status);
//         eprintln!("{:#?}", machine);
//         machines.push(machine);
//     }

//     eprintln!("999{:#?}", machines);

//     Ok(())
// }

// #[cfg(test)]
// mod tests {
//     use std::sync::Arc;
//
//     use crate::{library::cfg, miner::bootstrap::AppState};
//
//     const ACCOUNT_ID: i64 = 6192889942050345985;
//
//     const MAC1: &str = "28:E2:97:3E:6F:06";
//     const MAC2: &str = "28:E2:97:4D:1A:87";
//
//     #[tokio::test]
//     async fn test_get_machines() {
//         cfg::init(&"./fixtures/config.toml".to_string());
//         // let app_state = Arc::new(AppState::init().await);
//
//         // let mut redis = app_state.get_redis().await.unwrap();
//
//         // let res = get_machines(app_state, ACCOUNT_ID).await;
//
//         // eprintln!("{:#?}", res);
//     }
// }
