use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::types::Json;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
#[sqlx(rename_all = "lowercase")]
pub struct BwPool {
    pub pool_id: i64,
    pub account_id: i64,
    pub name: String,

    pub settings: Json<Vec<Setting>>,

    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct Setting {
    pub coin: String,
    pub user: String,
    pub password: String,
    pub url: String,
    pub worker: String,
    pub suffix: bool,
}
