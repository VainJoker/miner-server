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

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use sqlx::PgPool;

    use super::*;

    const ACCOUNT_ID: i64 = 6192889942050345985;
    const POLICY_ID_1: i64 = 6194821006046008321;
    const POLICY_ID_2: i64 = 6194821006046008322;

    #[sqlx::test(fixtures(path = "../../fixtures", scripts("users")))]
    async fn test_create_bw_policy(pool: PgPool) -> sqlx::Result<()> {
        let item = CreateBwPolicySchema {
            account_id: ACCOUNT_ID,
            name: "aaa".to_string(),
            settings: Some(vec![Setting {
                time: Utc::now().naive_utc(),
                mode: "test".to_string(),
            }]),
        };
        let a = BwPolicy::create_bw_policy(&pool, &item)
            .await
            .unwrap();
        assert_eq!(a.name, "aaa");

        Ok(())
    }

    #[sqlx::test(fixtures(
        path = "../../fixtures",
        scripts("users", "policy")
    ))]
    async fn test_fetch_policy_by_account_id(pool: PgPool) -> sqlx::Result<()> {
        let policies = BwPolicy::fetch_policy_by_account_id(&pool, ACCOUNT_ID)
            .await
            .unwrap();
        assert_eq!(policies.len(), 2);

        Ok(())
    }

    #[sqlx::test(fixtures(
        path = "../../fixtures",
        scripts("users", "policy")
    ))]
    async fn test_update_policy_by_policy_id(pool: PgPool) -> sqlx::Result<()> {
        let item = UpdateBwPolicySchema {
            policy_id: POLICY_ID_1,
            account_id: ACCOUNT_ID,
            name: Some("bbb".to_string()),
            settings: Some(vec![Setting {
                time: Utc::now().naive_utc(),
                mode: "test".to_string(),
            }]),
        };
        let rows_affected =
            BwPolicy::update_policy_by_policy_id(&pool, item)
                .await
                .unwrap();
        assert_eq!(rows_affected, 1);

        Ok(())
    }

    #[sqlx::test(fixtures(
        path = "../../fixtures",
        scripts("users", "policy")
    ))]
    async fn test_delete_policy_by_policy_id(pool: PgPool) -> sqlx::Result<()> {
        let item = DeleteBwPolicySchema {
            policy_id: POLICY_ID_1,
            account_id: ACCOUNT_ID,
        };
        let rows_affected =
            BwPolicy::delete_policy_by_policy_id(&pool, item)
                .await
                .unwrap();
        assert_eq!(rows_affected, 1);

        Ok(())
    }

    #[sqlx::test(fixtures(
        path = "../../fixtures",
        scripts("users", "policy")
    ))]
    async fn test_fetch_policy_count(pool: PgPool) -> sqlx::Result<()> {
        let count = BwPolicy::fetch_policy_count(&pool, ACCOUNT_ID)
            .await
            .unwrap();
        assert_eq!(count.unwrap(), 2);

        Ok(())
    }

    #[sqlx::test(fixtures(
        path = "../../fixtures",
        scripts("users", "policy")
    ))]
    async fn test_fetch_policy_info_by_ids(pool: PgPool) -> sqlx::Result<()> {
        let item = ReadBwPolicySchema {
            policy_ids: vec![POLICY_ID_1, POLICY_ID_2],
            account_id: ACCOUNT_ID,
        };
        let policies =
            BwPolicy::fetch_policy_info_by_ids(&pool, item)
                .await
                .unwrap();
        assert_eq!(policies.len(), 2);

        Ok(())
    }
}
