// use chrono::NaiveDateTime;
// use serde::{Deserialize, Serialize};
// use validator::ValidateRequired;
// use crate::library::error::{AppInnerError, InnerResult};
// use crate::library::DB;
// use crate::models::bw_account::CreateBwAccountSchema;
// use crate::models::CRUD;
// use crate::models::types::{AccountStatus, Currency, Language};
//
// #[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
// #[sqlx(rename_all = "lowercase")]
// pub struct BwGroup {
//     pub group_id: i64,
//     pub account_id: i64,
//     pub name: String,
//
//     pub remark: Option<String>,
//
//     pub created_at: NaiveDateTime,
//     pub updated_at: Option<NaiveDateTime>,
//     pub deleted_at: Option<NaiveDateTime>,
// }
