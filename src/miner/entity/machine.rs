use serde_derive::{Deserialize, Serialize};
use sqlx::types::Json;
use crate::library::error::AppResult;
use crate::library::Redis;
use crate::models::machine::Setting;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReadMachineResponse{
    mac: String,
    account_id: i64,

    mode: usize,
    now_rate: f64,
    avg_rate: f64,
    history_rate: Vec<f64>,
    power_mode: String,
    dig_time: i32,
    pool: Vec<Pool>,
    hard_err: f64,
    refuse: f64,
    temperature: String,
    fan: String,
    led: i32,

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

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Pool {
    url: String,
    user: String,
    legal: bool,
    active: bool,
    drag_id: i32,
    pool_priority: i32,
    pass: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Coin {
    algorithm: String,
    symbol: String,
}

pub async fn get_machines(
   redis: &mut Redis,
   account_id: i64
) -> AppResult<()>{
    let r_user_key = &format!("miner_user:{}",  account_id);

    let r_user_fields = redis.hkeys(r_user_key).await?.unwrap();
    eprintln!("{:#?}", r_user_fields);

    let r_status_keys: Vec<_> = r_user_fields
        .iter()
        .map(|k|{
                format!("miner_status:{}", k)
        }).collect();
    let r_status_keys:Vec<_> = r_status_keys.iter().map(|s| s.as_str()).collect();
    eprintln!("{:#?}", r_status_keys);

    let r_status_value = redis.hgetalls(&r_status_keys).await?;
    eprintln!("{:#?}", r_status_value);
    let none_indices: Vec<_> = r_status_value.iter().enumerate()
        .filter_map(|(i, &ref item)| {
            if item.is_empty() {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    eprintln!("{:#?}", none_indices);
    let none_machine_hash_keys: Vec<_> = none_indices.into_iter().map(|i| &r_user_fields[i]).collect();
    let none_machine_hash_keys:Vec<_> = none_machine_hash_keys.iter().map(|s| s.as_str()).collect();
    eprintln!("{:#?}",none_machine_hash_keys);
    let r_user_values = redis.hgets(r_user_key, &none_machine_hash_keys).await?;
    eprintln!("{:#?}",r_user_values);
    // let mut machines = Vec::new();

    // let online_machines = serde_json::from_str(r_status_value)?;
    //
    // machines.extend(r_status_value);
    // machines.extend(r_user_values);
    // machines.retain(|m| m.is_some());
    //
    // eprintln!("{:#?}", machines);

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use super::*;
    use crate::library::cfg;
    use crate::miner::bootstrap::AppState;

    const ACCOUNT_ID: i64 = 6192889942050345985;

    const MAC1: &str = "28:E2:97:3E:6F:06";
    const MAC2: &str = "28:E2:97:4D:1A:87";

    #[tokio::test]
    async fn test_get_machines() {
        cfg::init(&"./fixtures/config.toml".to_string());
        let app_state = Arc::new(AppState::init().await);

        let mut redis = app_state.get_redis().await.unwrap();

        let res = get_machines(&mut redis, ACCOUNT_ID).await;

        // eprintln!("{:#?}", res);
    }
}
