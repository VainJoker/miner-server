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

// TODO: option remark
#[derive(Debug, Deserialize)]
pub struct CreateBwGroupSchema {
    pub account_id: i64,
    pub name: String,
    pub remark: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBwGroupSchema {
    pub group_id: i64,
    pub account_id: i64,
    pub name: Option<String>,
    pub remark: Option<String>,
}

#[derive(Debug, Deserialize)]
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
        let map = sqlx::query_as!(
            Self,
            r#"
            INSERT INTO bw_group (account_id, name, remark) VALUES ($1, $2, $3)
            RETURNING group_id,account_id,name,remark,
            created_at,updated_at,deleted_at
            "#,
            item.account_id,
            item.name,
            item.remark
        );
        Ok(map.fetch_one(db).await?)
    }

    pub async fn fetch_group_by_account_id(
        db: &DB,
        account_id: i64,
    ) -> InnerResult<Vec<Self>> {
        let map = sqlx::query_as!(
            Self,
            r#"
            SELECT group_id,account_id,name,remark,
            created_at,updated_at,deleted_at
            FROM bw_group WHERE account_id = $1 AND deleted_at IS NULL
            "#,
            account_id
        );
        Ok(map.fetch_all(db).await?)
    }

    pub async fn update_group_by_group_id(
        db: &DB,
        item: UpdateBwGroupSchema,
    ) -> InnerResult<u64> {
        let map = sqlx::query_as!(
            Self,
            r#"
            UPDATE bw_group SET name = COALESCE($1, name), remark = COALESCE($2, remark)
            WHERE group_id = $3 AND account_id = $4 AND deleted_at IS NULL
            "#,
            item.name,
            item.remark,
            item.group_id,
            item.account_id
        );
        Ok(map.execute(db).await?.rows_affected())
    }

    pub async fn delete_group_by_group_id(
        db: &DB,
        item: DeleteBwGroupSchema,
    ) -> InnerResult<u64> {
        let map = sqlx::query_as!(
            Self,
            r#"
            UPDATE bw_group SET deleted_at = now()
            WHERE group_id = $1 AND account_id = $2 AND deleted_at IS NULL
            "#,
            item.group_id,
            item.account_id
        );
        Ok(map.execute(db).await?.rows_affected())
    }

    pub async fn fetch_group_count(
        db: &DB,
        account_id: i64,
    ) -> InnerResult<Option<i64>> {
        let map = sqlx::query_scalar!(
            r#"SELECT COUNT(*) FROM bw_group WHERE deleted_at IS NULL and account_id = $1"#,
            account_id
        );
        Ok(map.fetch_one(db).await?)
    }

    pub async fn fetch_group_info_by_ids(
        db: &DB,
        item: ReadBwGroupSchema,
    ) -> InnerResult<Vec<Self>> {
        let map = sqlx::query_as!(
            Self,
            r#"
            SELECT group_id,account_id,name,remark,
            created_at,updated_at,deleted_at
            FROM bw_group WHERE account_id = $1 AND group_id = ANY($2)
            "#,
            item.account_id,
            &item.group_ids
        );
        Ok(map.fetch_all(db).await?)
    }
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use super::*;

    #[ignore]
    #[sqlx::test(fixtures(path = "../../fixtures", scripts("users")))]
    async fn basic_test(pool: PgPool) -> sqlx::Result<()> {
        let new_bw_group = CreateBwGroupSchema {
            account_id: 6192889942050345985,
            name: "aaa".to_string(),
            remark: "aaa".to_string(),
        };
        let a = BwGroup::create_bw_group(&pool, &new_bw_group)
            .await
            .unwrap();
        assert_eq!(a.name, "aaa");
        // let foo = sqlx::query("SELECT * FROM foo")
        //     .fetch_one(&mut conn)
        //     .await?;

        // assert_eq!(foo.get::<String, _>("bar"), "foobar!");

        Ok(())
    }
}
