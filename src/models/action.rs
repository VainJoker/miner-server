use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::types::Action;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
#[sqlx(rename_all = "lowercase")]
pub struct BwAction {
    pub account_id: i64,
    pub mac: String,

    pub action: Action,

    pub remark: String,

    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}
