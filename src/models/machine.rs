use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::types::Json;

use crate::models::types::EnergyMode;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
#[sqlx(rename_all = "lowercase")]
pub struct BwMachine {
    pub mac: String,
    pub account_id: i64,

    pub device_type: String,
    pub device_name: String,
    pub device_ip: String,

    pub group_id: i64,
    pub policy_id: i64,
    pub template_id: i64,

    pub setting: Json<Setting>,

    pub hardware_version: String,
    pub software_version: String,

    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
#[sqlx(rename_all = "lowercase")]
pub struct Setting {
    pub crypto_coin: String,
    pub power_modes: Vec<EnergyMode>,
    pub pool_maximal: String,

    pub support_boot: bool,
    pub support_reset: bool,
    pub support_update: bool,
    pub support_led: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MachineStatus {
    pub mac: String,

    pub device_type: String,
    pub device_ip: String,

    pub current_rate: String,
    pub average_rate: String,
    pub history_rate: String,

    pub energy_mode: EnergyMode,
    pub dig_time: String,
    pub hard_err: String,
    pub refuse: String,

    pub device_temp: String,
    pub device_fan: String,
    pub device_status: String,
}
