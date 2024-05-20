use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::library::error::{AppInnerError, InnerResult};
use crate::models::bw_account::CreateBwAccountSchema;
use crate::models::CRUD;
use crate::models::types::{AccountStatus, Currency, Language};

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
#[sqlx(rename_all = "lowercase")]
pub struct BwGroup {
    pub group_id: i64,
    pub account_id: i64,
    pub name: String,

    pub remark: String,

    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

// #[CURD table_name]

// impl CRUD<PgPool> for BwGroup {
//     type Error = AppInnerError;
//
//     async fn create(&self, db: &PgPool, item: &Self) -> Result<Self, Self::Error> {
//         let map = sqlx::query_as!(
//             Self,
//             r#"
//             INSERT INTO bw_group (account_id, name, remark) VALUES ($1, $2, $3)
//             RETURNING group_id,account_id,name,remark,
//             created_at,updated_at,deleted_at
//             "#,
//             item.account_id,
//             item.name,
//             item.remark
//         );
//         Ok(map.fetch_one(db).await?)
//     }
//
//     fn read(&self, db: &PgPool) -> Result<Vec<Self>, Self::Error> where Self: Sized {
//         todo!()
//     }
//
//     fn update(&self, db: &PgPool, item: &Self) -> Result<(), Self::Error> {
//         todo!()
//     }
//
//     fn delete(&self, db: &PgPool, item: &Self) -> Result<(), Self::Error> {
//         todo!()
//     }
// }