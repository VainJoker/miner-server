use serde_derive::{Deserialize, Serialize};

use crate::library::{error::InnerResult, DB};

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
#[sqlx(rename_all = "lowercase")]
pub struct BwAccountSetting {
    pub account_id: i64,
    pub key: String,
}

impl BwAccountSetting {
    pub async fn create_bw_account_setting(
        db: &DB,
        item: &BwAccountSetting,
    ) -> InnerResult<Self> {
        let sql = r#"
            INSERT INTO bw_account_setting (account_id, key) VALUES ($1, $2)
            RETURNING account_id, key
            "#;
        let map = sqlx::query_as(sql).bind(item.account_id).bind(&item.key);
        Ok(map.fetch_one(db).await?)
    }

    pub async fn fetch_key_by_account_id(
        db: &DB,
        account_id: i64,
    ) -> InnerResult<String> {
        let sql = r#"
        SELECT key
        FROM bw_account_setting WHERE account_id = $1
        "#;
        let map = sqlx::query_scalar(sql).bind(account_id);
        Ok(map.fetch_one(db).await?)
    }

    pub async fn fetch_account_id_by_key(
        db: &DB,
        key: &str,
    ) -> InnerResult<i64> {
        let sql = r#"
        SELECT account_id
        FROM bw_account_setting WHERE key = $1
        "#;
        let map = sqlx::query_scalar(sql).bind(key);
        Ok(map.fetch_one(db).await?)
    }
}
