use serde::{Deserialize, Serialize};

#[derive(sqlx::Type, Debug, Clone, Copy, Serialize, Deserialize)]
#[sqlx(type_name = "currency")]
pub enum Currency {
    USD,
    EUR,
    GBP,
    CNY,
}

#[derive(sqlx::Type, Debug, Clone, Copy, Serialize, Deserialize)]
#[sqlx(type_name = "language")]
pub enum Language {
    #[sqlx(rename = "en-US")]
    EnUs,
    #[sqlx(rename = "zh-CN")]
    ZhCn,
    #[sqlx(rename = "fr-FR")]
    FrFr,
    #[sqlx(rename = "es-ES")]
    EsEs,
}

#[derive(
    sqlx::Type,
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    PartialOrd,
    PartialEq,
)]
#[sqlx(type_name = "account_status")]
#[sqlx(rename_all = "lowercase")]
pub enum AccountStatus {
    #[sqlx(rename = "active")]
    Active,
    #[sqlx(rename = "inactive")]
    Inactive,
    #[sqlx(rename = "suspended")]
    Suspend,
}

#[derive(
    sqlx::Type,
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    PartialOrd,
    PartialEq,
)]
#[sqlx(rename_all = "lowercase")]
pub enum Action {
    Restart,
    SendMiner,
    SetGroup,
    SetPool,
    SetPowerMode,
    SetLED,
    ResetToFactory,
    Offline,
    Upgrade,
    Delete,
}

#[derive(
    sqlx::Type,
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    PartialOrd,
    PartialEq,
)]
#[sqlx(rename_all = "lowercase")]
pub enum EnergyMode {
    Power,
    Idle,
    Balance,
    Economize,
}
