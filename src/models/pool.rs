use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::types::Json;

use crate::library::{error::InnerResult, DB};

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

#[derive(Debug, Deserialize)]
pub struct CreateBwPoolSchema {
    pub account_id: i64,
    pub name: String,
    pub settings: Option<Vec<Setting>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBwPoolSchema {
    pub pool_id: i64,
    pub account_id: i64,
    pub name: Option<String>,
    pub settings: Option<Vec<Setting>>,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct DeleteBwPoolSchema {
    pub pool_id: i64,
    pub account_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct ReadBwPoolSchema {
    pub pool_ids: Vec<i64>,
    pub account_id: i64,
}

impl BwPool {
    pub async fn create_bw_pool(
        db: &DB,
        item: &CreateBwPoolSchema,
    ) -> InnerResult<Self> {
        let sql = r#"
        INSERT INTO bw_pool (account_id, name, settings) VALUES ($1, $2,
    $3)     RETURNING pool_id,account_id,name,settings,
        created_at,updated_at,deleted_at
        "#;
        let map = sqlx::query_as(sql)
            .bind(item.account_id)
            .bind(&item.name)
            .bind(Json(&item.settings));
        Ok(map.fetch_one(db).await?)
    }

    pub async fn fetch_pool_by_account_id(
        db: &DB,
        account_id: i64,
    ) -> InnerResult<Vec<Self>> {
        let sql = r#"
        SELECT pool_id,account_id,name,settings,
        created_at,updated_at,deleted_at
        FROM bw_pool WHERE account_id = $1 AND deleted_at IS NULL
        "#;
        let map = sqlx::query_as(sql).bind(account_id);
        Ok(map.fetch_all(db).await?)
    }

    pub async fn update_pool_by_pool_id(
        db: &DB,
        item: UpdateBwPoolSchema,
    ) -> InnerResult<u64> {
        let sql = r#"
        UPDATE bw_pool SET name = COALESCE($1, name), settings = $2
        WHERE pool_id = $3 AND account_id = $4
        AND deleted_at IS NULL"#;
        let map = sqlx::query(sql)
            .bind(&item.name)
            .bind(Json(&item.settings))
            .bind(item.pool_id)
            .bind(item.account_id);
        Ok(map.execute(db).await?.rows_affected())
    }

    pub async fn delete_pool_by_pool_id(
        db: &DB,
        item: DeleteBwPoolSchema,
    ) -> InnerResult<u64> {
        let sql = r#"
        UPDATE bw_pool SET deleted_at = now()
        WHERE pool_id = $1 AND account_id = $2 AND deleted_at IS NULL
        "#;
        let map = sqlx::query(sql).bind(item.pool_id).bind(item.account_id);
        Ok(map.execute(db).await?.rows_affected())
    }

    pub async fn fetch_pool_count(
        db: &DB,
        account_id: i64,
    ) -> InnerResult<Option<i64>> {
        let sql = r#"SELECT COUNT(*) FROM bw_pool WHERE deleted_at IS NULL
    and account_id = $1"#;
        let map = sqlx::query_scalar(sql).bind(account_id);
        Ok(map.fetch_one(db).await?)
    }

    pub async fn fetch_pool_info_by_ids(
        db: &DB,
        item: ReadBwPoolSchema,
    ) -> InnerResult<Vec<Self>> {
        let sql = r#"
        SELECT pool_id,account_id,name,settings,
        created_at,updated_at,deleted_at
        FROM bw_pool WHERE account_id = $1 AND pool_id = ANY($2)
        "#;
        let map = sqlx::query_as(sql)
            .bind(item.account_id)
            .bind(&item.pool_ids);
        Ok(map.fetch_all(db).await?)
    }
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;
    use BwPool;
    use CreateBwPoolSchema;
    use DeleteBwPoolSchema;
    use ReadBwPoolSchema;
    use UpdateBwPoolSchema;

    use super::*;

    const ACCOUNT_ID: i64 = 6192889942050345985;
    const POLICY_ID_1: i64 = 6194824969470350666;
    const POLICY_ID_2: i64 = 6194824969470350667;

    #[sqlx::test(fixtures(path = "../../fixtures", scripts("users")))]
    async fn test_create_bw_pool(pool: PgPool) -> sqlx::Result<()> {
        let item = CreateBwPoolSchema {
            account_id: ACCOUNT_ID,
            name: "aaa".to_string(),
            settings: Some(vec![Setting {
                coin: "BTC".to_string(),
                user: "JohnDoe".to_string(),
                password: "jd1234".to_string(),
                url: "https://api.blockchain.com".to_string(),
                worker: "worker1".to_string(),
                suffix: true,
            }]),
        };
        let a = BwPool::create_bw_pool(&pool, &item).await.unwrap();
        assert_eq!(a.name, "aaa");

        Ok(())
    }

    #[sqlx::test(fixtures(path = "../../fixtures", scripts("users", "pool")))]
    async fn test_fetch_pool_by_account_id(pool: PgPool) -> sqlx::Result<()> {
        let policies = BwPool::fetch_pool_by_account_id(&pool, ACCOUNT_ID)
            .await
            .unwrap();
        assert_eq!(policies.len(), 2);

        Ok(())
    }

    #[sqlx::test(fixtures(path = "../../fixtures", scripts("users", "pool")))]
    async fn test_update_pool_by_pool_id(pool: PgPool) -> sqlx::Result<()> {
        let item = UpdateBwPoolSchema {
            pool_id: POLICY_ID_1,
            account_id: ACCOUNT_ID,
            name: Some("bbb".to_string()),
            settings: Some(vec![Setting {
                coin: "BTC".to_string(),
                user: "JohnDoe".to_string(),
                password: "jd1234".to_string(),
                url: "https://api.blockchain.com".to_string(),
                worker: "worker1".to_string(),
                suffix: true,
            }]),
        };
        let rows_affected =
            BwPool::update_pool_by_pool_id(&pool, item)
                .await
                .unwrap();
        assert_eq!(rows_affected, 1);

        Ok(())
    }

    #[sqlx::test(fixtures(path = "../../fixtures", scripts("users", "pool")))]
    async fn test_delete_pool_by_pool_id(pool: PgPool) -> sqlx::Result<()> {
        let item = DeleteBwPoolSchema {
            pool_id: POLICY_ID_1,
            account_id: ACCOUNT_ID,
        };
        let rows_affected =
            BwPool::delete_pool_by_pool_id(&pool, item)
                .await
                .unwrap();
        assert_eq!(rows_affected, 1);

        Ok(())
    }

    #[sqlx::test(fixtures(path = "../../fixtures", scripts("users", "pool")))]
    async fn test_fetch_pool_count(pool: PgPool) -> sqlx::Result<()> {
        let count = BwPool::fetch_pool_count(&pool, ACCOUNT_ID).await.unwrap();
        assert_eq!(count.unwrap(), 2);

        Ok(())
    }

    #[sqlx::test(fixtures(path = "../../fixtures", scripts("users", "pool")))]
    async fn test_fetch_pool_info_by_ids(pool: PgPool) -> sqlx::Result<()> {
        let item = ReadBwPoolSchema {
            pool_ids: vec![POLICY_ID_1, POLICY_ID_2],
            account_id: ACCOUNT_ID,
        };
        let policies = BwPool::fetch_pool_info_by_ids(&pool, item)
            .await
            .unwrap();
        assert_eq!(policies.len(), 2);

        Ok(())
    }
}
