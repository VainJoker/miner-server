use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::types::Json;

use crate::library::{error::InnerResult, DB};

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
#[sqlx(rename_all = "lowercase")]
pub struct BwPolicy {
    pub policy_id: i64,
    pub account_id: i64,
    pub name: String,

    pub settings: Json<Vec<Setting>>,

    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct Setting {
    pub time: NaiveDateTime,
    pub mode: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateBwPolicySchema {
    pub account_id: i64,
    pub name: String,
    pub settings: Option<Vec<Setting>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBwPolicySchema {
    pub policy_id: i64,
    pub account_id: i64,
    pub name: Option<String>,
    pub settings: Option<Vec<Setting>>,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct DeleteBwPolicySchema {
    pub policy_id: i64,
    pub account_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct ReadBwPolicySchema {
    pub policy_ids: Vec<i64>,
    pub account_id: i64,
}

impl BwPolicy {
    pub async fn create_bw_policy(
        db: &DB,
        item: &CreateBwPolicySchema,
    ) -> InnerResult<Self> {
        let sql = r#"
        INSERT INTO bw_policy (account_id, name, settings) VALUES ($1, $2,
    $3)     RETURNING policy_id,account_id,name,settings,
        created_at,updated_at,deleted_at
        "#;
        let map = sqlx::query_as(sql)
            .bind(item.account_id)
            .bind(&item.name)
            .bind(Json(&item.settings));
        Ok(map.fetch_one(db).await?)
    }

    pub async fn fetch_policy_by_account_id(
        db: &DB,
        account_id: i64,
    ) -> InnerResult<Vec<Self>> {
        let sql = r#"
        SELECT policy_id,account_id,name,settings,
        created_at,updated_at,deleted_at
        FROM bw_policy WHERE account_id = $1 AND deleted_at IS NULL
        "#;
        let map = sqlx::query_as(sql).bind(account_id);
        Ok(map.fetch_all(db).await?)
    }

    pub async fn update_policy_by_policy_id(
        db: &DB,
        item: UpdateBwPolicySchema,
    ) -> InnerResult<u64> {
        let sql = r#"
        UPDATE bw_policy SET name = COALESCE($1, name), settings = $2
        WHERE policy_id = $3 AND account_id = $4
        AND deleted_at IS NULL"#;
        let map = sqlx::query(sql)
            .bind(&item.name)
            .bind(Json(&item.settings))
            .bind(item.policy_id)
            .bind(item.account_id);
        Ok(map.execute(db).await?.rows_affected())
    }

    pub async fn delete_policy_by_policy_id(
        db: &DB,
        item: DeleteBwPolicySchema,
    ) -> InnerResult<u64> {
        let sql = r#"
        UPDATE bw_policy SET deleted_at = now()
        WHERE policy_id = $1 AND account_id = $2 AND deleted_at IS NULL
        "#;
        let map = sqlx::query(sql).bind(item.policy_id).bind(item.account_id);
        Ok(map.execute(db).await?.rows_affected())
    }

    pub async fn fetch_policy_count(
        db: &DB,
        account_id: i64,
    ) -> InnerResult<Option<i64>> {
        let sql = r#"SELECT COUNT(*) FROM bw_policy WHERE deleted_at IS NULL
    and account_id = $1"#;
        let map = sqlx::query_scalar(sql).bind(account_id);
        Ok(map.fetch_one(db).await?)
    }

    pub async fn fetch_policy_info_by_ids(
        db: &DB,
        item: ReadBwPolicySchema,
    ) -> InnerResult<Vec<Self>> {
        let sql = r#"
        SELECT policy_id,account_id,name,settings,
        created_at,updated_at,deleted_at
        FROM bw_policy WHERE account_id = $1 AND policy_id = ANY($2)
        "#;
        let map = sqlx::query_as(sql)
            .bind(item.account_id)
            .bind(&item.policy_ids);
        Ok(map.fetch_all(db).await?)
    }
}
