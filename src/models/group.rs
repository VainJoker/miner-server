use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::library::{error::InnerResult, DB};

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
#[sqlx(rename_all = "lowercase")]
pub struct BwGroup {
    pub group_id: i64,
    pub account_id: i64,
    pub name: String,

    pub remark: Option<String>,

    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct CreateBwGroupSchema {
    pub account_id: i64,
    pub name: String,
    pub remark: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBwGroupSchema {
    pub group_id: i64,
    pub account_id: i64,
    pub name: Option<String>,
    pub remark: Option<String>,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct DeleteBwGroupSchema {
    pub group_id: i64,
    pub account_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct ReadBwGroupSchema {
    pub group_ids: Vec<i64>,
    pub account_id: i64,
}

impl BwGroup {
    pub async fn create_bw_group(
        db: &DB,
        item: &CreateBwGroupSchema,
    ) -> InnerResult<Self> {
        let sql = r#"
            INSERT INTO bw_group (account_id, name, remark) VALUES ($1, $2, $3)
            RETURNING group_id,account_id,name,remark,
            created_at,updated_at,deleted_at
            "#;
        let map = sqlx::query_as(sql)
            .bind(item.account_id)
            .bind(&item.name)
            .bind(&item.remark);
        Ok(map.fetch_one(db).await?)
    }

    pub async fn fetch_group_by_account_id(
        db: &DB,
        account_id: i64,
    ) -> InnerResult<Vec<Self>> {
        let sql = r#"
        SELECT group_id,account_id,name,remark,
        created_at,updated_at,deleted_at
        FROM bw_group WHERE account_id = $1 AND deleted_at IS NULL
        "#;
        let map = sqlx::query_as(sql).bind(account_id);
        Ok(map.fetch_all(db).await?)
    }

    pub async fn update_group_by_group_id(
        db: &DB,
        item: &UpdateBwGroupSchema,
    ) -> InnerResult<u64> {
        let sql = r#"
        UPDATE bw_group SET name = COALESCE($1, name), remark = COALESCE($2, remark)
        WHERE group_id = $3 AND account_id = $4 AND deleted_at IS NULL
        "#;
        let map = sqlx::query(sql)
            .bind(&item.name)
            .bind(&item.remark)
            .bind(item.group_id)
            .bind(item.account_id);
        Ok(map.execute(db).await?.rows_affected())
    }

    pub async fn delete_group_by_group_id(
        db: &DB,
        item: &DeleteBwGroupSchema,
    ) -> InnerResult<u64> {
        let sql = r#"
        UPDATE bw_group SET deleted_at = now()
        WHERE group_id = $1 AND account_id = $2 AND deleted_at IS NULL
        "#;
        let map = sqlx::query(sql).bind(item.group_id).bind(item.account_id);
        Ok(map.execute(db).await?.rows_affected())
    }

    pub async fn fetch_group_count(
        db: &DB,
        account_id: i64,
    ) -> InnerResult<Option<i64>> {
        let sql = r#"SELECT COUNT(*) FROM bw_group WHERE deleted_at IS NULL and account_id = $1"#;
        let map = sqlx::query_scalar(sql).bind(account_id);
        Ok(map.fetch_one(db).await?)
    }

    pub async fn fetch_group_info_by_ids(
        db: &DB,
        item: &ReadBwGroupSchema,
    ) -> InnerResult<Vec<Self>> {
        let sql = r#"
        SELECT group_id,account_id,name,remark,
        created_at,updated_at,deleted_at
        FROM bw_group WHERE account_id = $1 AND group_id = ANY($2)
        "#;
        let map = sqlx::query_as(sql)
            .bind(item.account_id)
            .bind(&item.group_ids);
        Ok(map.fetch_all(db).await?)
    }
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use super::*;

    const ACCOUNT_ID: i64 = 6192889942050345985;
    const GROUP_ID_1: i64 = 6193003777960711169;
    const GROUP_ID_2: i64 = 6193003777960711170;

    #[sqlx::test(fixtures(path = "../../fixtures", scripts("users")))]
    async fn test_create_group(pool: PgPool) -> sqlx::Result<()> {
        let new_bw_group = CreateBwGroupSchema {
            account_id: ACCOUNT_ID,
            name: "aaa".to_string(),
            remark: Some("aaa".to_string()),
        };
        let a = BwGroup::create_bw_group(&pool, &new_bw_group)
            .await
            .unwrap();
        assert_eq!(a.name, "aaa");

        Ok(())
    }

    #[sqlx::test(fixtures(
        path = "../../fixtures",
        scripts("users", "groups")
    ))]
    async fn test_fetch_group_by_account_id(pool: PgPool) -> sqlx::Result<()> {
        let groups = BwGroup::fetch_group_by_account_id(&pool, ACCOUNT_ID)
            .await
            .unwrap();
        assert_eq!(groups.len(), 2);

        Ok(())
    }

    #[sqlx::test(fixtures(
        path = "../../fixtures",
        scripts("users", "groups")
    ))]
    async fn test_update_group_by_group_id(pool: PgPool) -> sqlx::Result<()> {
        let update_bw_group = UpdateBwGroupSchema {
            group_id: GROUP_ID_1,
            account_id: ACCOUNT_ID,
            name: Some("bbb".to_string()),
            remark: Some("bbb".to_string()),
        };
        let rows_affected =
            BwGroup::update_group_by_group_id(&pool, &update_bw_group)
                .await
                .unwrap();
        assert_eq!(rows_affected, 1);

        Ok(())
    }

    #[sqlx::test(fixtures(
        path = "../../fixtures",
        scripts("users", "groups")
    ))]
    async fn test_delete_group_by_group_id(pool: PgPool) -> sqlx::Result<()> {
        let delete_bw_group = DeleteBwGroupSchema {
            group_id: GROUP_ID_1,
            account_id: ACCOUNT_ID,
        };
        let rows_affected =
            BwGroup::delete_group_by_group_id(&pool, &delete_bw_group)
                .await
                .unwrap();
        assert_eq!(rows_affected, 1);

        Ok(())
    }

    #[sqlx::test(fixtures(
        path = "../../fixtures",
        scripts("users", "groups")
    ))]
    async fn test_fetch_group_count(pool: PgPool) -> sqlx::Result<()> {
        let count =
            BwGroup::fetch_group_count(&pool, ACCOUNT_ID).await.unwrap();
        assert_eq!(count.unwrap(), 2);
        let new_bw_group = CreateBwGroupSchema {
            account_id: ACCOUNT_ID,
            name: "aaa".to_string(),
            remark: Some("aaa".to_string()),
        };
        BwGroup::create_bw_group(&pool, &new_bw_group)
            .await
            .unwrap();
        let count =
            BwGroup::fetch_group_count(&pool, ACCOUNT_ID).await.unwrap();
        assert_eq!(count.unwrap(), 3);
        let delete_bw_group = DeleteBwGroupSchema {
            group_id: GROUP_ID_1,
            account_id: ACCOUNT_ID,
        };
        BwGroup::delete_group_by_group_id(&pool, &delete_bw_group)
            .await
            .unwrap();
        let count =
            BwGroup::fetch_group_count(&pool, ACCOUNT_ID).await.unwrap();
        assert_eq!(count.unwrap(), 2);

        Ok(())
    }

    #[sqlx::test(fixtures(
        path = "../../fixtures",
        scripts("users", "groups")
    ))]
    async fn test_fetch_group_info_by_ids(pool: PgPool) -> sqlx::Result<()> {
        let read_bw_group = ReadBwGroupSchema {
            group_ids: vec![GROUP_ID_1, GROUP_ID_2],
            account_id: ACCOUNT_ID,
        };
        let groups = BwGroup::fetch_group_info_by_ids(&pool, &read_bw_group)
            .await
            .unwrap();
        assert_eq!(groups.len(), 2);

        Ok(())
    }

    #[sqlx::test(fixtures(
        path = "../../fixtures",
        scripts("users", "groups")
    ))]
    async fn test_create_group_with_invalid_input(
        pool: PgPool,
    ) -> sqlx::Result<()> {
        let new_bw_group = CreateBwGroupSchema {
            account_id: 0,        // Nonexistent account_id
            name: "".to_string(), // Empty name
            remark: Some("aaa".to_string()),
        };
        let result = BwGroup::create_bw_group(&pool, &new_bw_group).await;
        assert!(result.is_err());

        Ok(())
    }

    #[sqlx::test(fixtures(
        path = "../../fixtures",
        scripts("users", "groups")
    ))]
    async fn test_fetch_group_by_nonexistent_account_id(
        pool: PgPool,
    ) -> sqlx::Result<()> {
        let groups = BwGroup::fetch_group_by_account_id(&pool, 0) // Nonexistent account_id
            .await
            .unwrap();
        assert!(groups.is_empty());

        Ok(())
    }

    #[sqlx::test(fixtures(
        path = "../../fixtures",
        scripts("users", "groups")
    ))]
    async fn test_update_nonexistent_group(pool: PgPool) -> sqlx::Result<()> {
        let update_bw_group = UpdateBwGroupSchema {
            group_id: 0, // Nonexistent group_id
            account_id: ACCOUNT_ID,
            name: Some("bbb".to_string()),
            remark: Some("bbb".to_string()),
        };
        let rows_affected =
            BwGroup::update_group_by_group_id(&pool, &update_bw_group)
                .await
                .unwrap();
        assert_eq!(rows_affected, 0);

        Ok(())
    }

    #[sqlx::test(fixtures(
        path = "../../fixtures",
        scripts("users", "groups")
    ))]
    async fn test_delete_already_deleted_group(
        pool: PgPool,
    ) -> sqlx::Result<()> {
        let delete_bw_group = DeleteBwGroupSchema {
            group_id: GROUP_ID_1,
            account_id: ACCOUNT_ID,
        };
        BwGroup::delete_group_by_group_id(&pool, &delete_bw_group)
            .await
            .unwrap();
        let rows_affected =
            BwGroup::delete_group_by_group_id(&pool, &delete_bw_group)
                .await
                .unwrap();
        assert_eq!(rows_affected, 0);

        Ok(())
    }

    #[sqlx::test(fixtures(
        path = "../../fixtures",
        scripts("users", "groups")
    ))]
    async fn test_fetch_group_count_and_info_with_empty_group_list(
        pool: PgPool,
    ) -> sqlx::Result<()> {
        let new_account_id = 123456789; // An account_id with no groups
        let count = BwGroup::fetch_group_count(&pool, new_account_id)
            .await
            .unwrap();
        assert_eq!(count.unwrap(), 0);
        let read_bw_group = ReadBwGroupSchema {
            group_ids: vec![],
            account_id: new_account_id,
        };
        let groups = BwGroup::fetch_group_info_by_ids(&pool, &read_bw_group)
            .await
            .unwrap();
        assert!(groups.is_empty());

        Ok(())
    }
}
