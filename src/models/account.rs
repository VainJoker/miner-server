use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

use crate::{
    library::{error::InnerResult, DB},
    models::types::{AccountStatus, Currency, Language},
};

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
#[sqlx(rename_all = "lowercase")]
pub struct BwAccount {
    pub account_id: i64,
    pub name: String,
    pub email: String,
    pub email_verified_at: Option<NaiveDateTime>,
    pub password: String,
    pub failed_attempt: i32,
    pub status: AccountStatus,
    pub last_login: Option<NaiveDateTime>,

    pub local_currency: Currency,
    pub system_lang: Language,

    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct ResetPasswordSchema {
    pub account_id: i64,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateBwAccountSchema {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl BwAccount {
    pub async fn register_account(
        db: &DB,
        item: &CreateBwAccountSchema,
    ) -> InnerResult<Self> {
        let sql = r#"
            INSERT INTO bw_account (name, email, password) VALUES ($1, $2, $3)
            RETURNING account_id,name,email,email_verified_at,password,
            local_currency,system_lang ,
            status, failed_attempt, last_login,
            created_at,updated_at,deleted_at
            "#;
        let map = sqlx::query_as(sql)
            .bind(&item.name)
            .bind(&item.email)
            .bind(&item.password);

        Ok(map.fetch_one(db).await?)
    }

    pub async fn check_user_exists_by_email(
        db: &DB,
        email: &str,
    ) -> InnerResult<Option<bool>> {
        let sql = r#"SELECT EXISTS(SELECT 1 FROM bw_account WHERE email = $1)"#;
        let map = sqlx::query_scalar(sql).bind(email);
        Ok(map.fetch_one(db).await?)
    }

    pub async fn check_user_exists_by_account_id(
        db: &DB,
        account_id: &i64,
    ) -> InnerResult<Option<bool>> {
        let sql =
            r#"SELECT EXISTS(SELECT 1 FROM bw_account WHERE account_id = $1)"#;
        let map = sqlx::query_scalar(sql).bind(account_id);
        Ok(map.fetch_one(db).await?)
    }

    pub async fn fetch_user_by_email_or_name(
        db: &DB,
        email_or_name: &str,
    ) -> InnerResult<Vec<Self>> {
        let sql = r#"SELECT account_id,name,email,email_verified_at,password,
            local_currency,system_lang ,
            status, failed_attempt, last_login,
            created_at,updated_at,deleted_at
            FROM bw_account WHERE name = $1 or email = $1"#;
        let map = sqlx::query_as(sql).bind(email_or_name);

        Ok(map.fetch_all(db).await?)
    }

    pub async fn fetch_user_by_account_id(
        db: &DB,
        account_id: i64,
    ) -> InnerResult<Option<Self>> {
        let sql = r#"SELECT account_id,name,email,email_verified_at,password,
            local_currency, system_lang, status, failed_attempt, last_login,
            created_at,updated_at,deleted_at
            FROM bw_account WHERE account_id = $1"#;

        let map = sqlx::query_as(sql).bind(account_id);
        Ok(map.fetch_optional(db).await?)
    }

    pub async fn fetch_user_by_email(
        db: &DB,
        email: &str,
    ) -> InnerResult<Option<Self>> {
        let sql = r#"SELECT account_id,name,email,email_verified_at,password,
            local_currency,system_lang ,
            status , failed_attempt, last_login,
            created_at,updated_at,deleted_at
            FROM bw_account WHERE email = $1"#;
        let map = sqlx::query_as(sql).bind(email);
        Ok(map.fetch_optional(db).await?)
    }

    pub async fn update_last_login(
        db: &DB,
        account_id: i64,
    ) -> InnerResult<u64> {
        let map = sqlx::query(
            r#"UPDATE bw_account SET last_login = now()
        WHERE account_id = $1"#,
        )
        .bind(account_id);
        Ok(map.execute(db).await?.rows_affected())
    }

    pub async fn update_email_verified_at(
        db: &DB,
        account_id: i64,
    ) -> InnerResult<u64> {
        let map = sqlx::query(
            r#"UPDATE bw_account set email_verified_at = now(), status = 'active'
        WHERE account_id = $1"#,
        )
            .bind(account_id);
        Ok(map.execute(db).await?.rows_affected())
    }

    pub async fn update_password_by_account_id(
        db: &DB,
        item: &ResetPasswordSchema,
    ) -> InnerResult<u64> {
        let map = sqlx::query(
            r#"UPDATE bw_account set password = $1
        WHERE account_id = $2"#,
        )
        .bind(&item.password)
        .bind(item.account_id);
        Ok(map.execute(db).await?.rows_affected())
    }

    pub async fn check_user_active_by_account_id(
        db: &DB,
        account_id: i64,
    ) -> InnerResult<Option<bool>> {
        let map = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM bw_account WHERE account_id = $1 and status = 'active')",
        )
            .bind(account_id);
        Ok(map.fetch_one(db).await?)
    }
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use super::*;

    const ACCOUNT_ID: i64 = 6192889942050345985;
    const EMAIL: &str = "test@test.com";
    const MY_EMAIL: &str = "vainjoker@tuta.io";
    const NAME: &str = "Test User";
    const PASSWORD: &str = "password";
    const NONEXISTENT_ACCOUNT_ID: i64 = 0;
    const NONEXISTENT_EMAIL: &str = "nonexistent@test.com";

    #[sqlx::test(fixtures(path = "../../fixtures", scripts("users")))]
    async fn test_register_account(pool: PgPool) -> sqlx::Result<()> {
        let new_account = CreateBwAccountSchema {
            name: NAME.to_string(),
            email: EMAIL.to_string(),
            password: PASSWORD.to_string(),
        };
        let account = BwAccount::register_account(&pool, &new_account)
            .await
            .unwrap();
        assert_eq!(account.email, EMAIL);
        assert_eq!(account.name, NAME);

        Ok(())
    }

    #[sqlx::test(fixtures(path = "../../fixtures", scripts("users")))]
    async fn test_fetch_user_by_email(pool: PgPool) -> sqlx::Result<()> {
        let account = BwAccount::fetch_user_by_email(&pool, MY_EMAIL)
            .await
            .unwrap();
        assert_eq!(account.unwrap().email, MY_EMAIL);

        Ok(())
    }

    #[sqlx::test(fixtures(path = "../../fixtures", scripts("users")))]
    async fn test_fetch_user_by_account_id(pool: PgPool) -> sqlx::Result<()> {
        let account = BwAccount::fetch_user_by_account_id(&pool, ACCOUNT_ID)
            .await
            .unwrap();
        assert_eq!(account.unwrap().account_id, ACCOUNT_ID);

        Ok(())
    }

    #[sqlx::test(fixtures(path = "../../fixtures", scripts("users")))]
    async fn test_check_user_exists_by_email(pool: PgPool) -> sqlx::Result<()> {
        let exists = BwAccount::check_user_exists_by_email(&pool, MY_EMAIL)
            .await
            .unwrap();
        assert!(exists.unwrap());

        Ok(())
    }

    #[sqlx::test(fixtures(path = "../../fixtures", scripts("users")))]
    async fn test_check_user_exists_by_account_id(
        pool: PgPool,
    ) -> sqlx::Result<()> {
        let exists =
            BwAccount::check_user_exists_by_account_id(&pool, &ACCOUNT_ID)
                .await
                .unwrap();
        assert!(exists.unwrap());

        Ok(())
    }

    #[sqlx::test(fixtures(path = "../../fixtures", scripts("users")))]
    async fn test_check_user_active_by_account_id(
        pool: PgPool,
    ) -> sqlx::Result<()> {
        let is_active =
            BwAccount::check_user_active_by_account_id(&pool, ACCOUNT_ID)
                .await
                .unwrap();
        assert!(!is_active.unwrap()); // Assuming the account is active

        Ok(())
    }

    #[sqlx::test(fixtures(path = "../../fixtures", scripts("users")))]
    async fn test_update_last_login(pool: PgPool) -> sqlx::Result<()> {
        let rows_affected = BwAccount::update_last_login(&pool, ACCOUNT_ID)
            .await
            .unwrap();
        assert_eq!(rows_affected, 1);

        Ok(())
    }

    #[sqlx::test(fixtures(path = "../../fixtures", scripts("users")))]
    async fn test_update_email_verified_at(pool: PgPool) -> sqlx::Result<()> {
        let rows_affected =
            BwAccount::update_email_verified_at(&pool, ACCOUNT_ID)
                .await
                .unwrap();
        assert_eq!(rows_affected, 1);
        let is_active =
            BwAccount::check_user_active_by_account_id(&pool, ACCOUNT_ID)
                .await
                .unwrap();
        assert!(is_active.unwrap()); // Assuming the account is active

        Ok(())
    }

    #[sqlx::test(fixtures(path = "../../fixtures", scripts("users")))]
    async fn test_update_password_by_account_id(
        pool: PgPool,
    ) -> sqlx::Result<()> {
        let new_password = ResetPasswordSchema {
            account_id: ACCOUNT_ID,
            password: "new_password".to_string(),
        };
        let rows_affected =
            BwAccount::update_password_by_account_id(&pool, &new_password)
                .await
                .unwrap();
        assert_eq!(rows_affected, 1);

        Ok(())
    }

    #[sqlx::test(fixtures(path = "../../fixtures", scripts("users")))]
    async fn test_register_account_with_existing_email(
        pool: PgPool,
    ) -> sqlx::Result<()> {
        let new_account = CreateBwAccountSchema {
            name: "New User".to_string(),
            email: MY_EMAIL.to_string(),
            password: "password".to_string(),
        };
        let result = BwAccount::register_account(&pool, &new_account).await;
        assert!(result.is_err());

        Ok(())
    }

    #[sqlx::test(fixtures(path = "../../fixtures", scripts("users")))]
    async fn test_fetch_user_by_nonexistent_email(
        pool: PgPool,
    ) -> sqlx::Result<()> {
        let account = BwAccount::fetch_user_by_email(&pool, NONEXISTENT_EMAIL)
            .await
            .unwrap();
        assert!(account.is_none());

        Ok(())
    }

    #[sqlx::test(fixtures(path = "../../fixtures", scripts("users")))]
    async fn test_fetch_user_by_nonexistent_account_id(
        pool: PgPool,
    ) -> sqlx::Result<()> {
        let account =
            BwAccount::fetch_user_by_account_id(&pool, NONEXISTENT_ACCOUNT_ID)
                .await
                .unwrap();
        assert!(account.is_none());

        Ok(())
    }

    #[sqlx::test(fixtures(path = "../../fixtures", scripts("users")))]
    async fn test_check_user_exists_by_nonexistent_email(
        pool: PgPool,
    ) -> sqlx::Result<()> {
        let exists =
            BwAccount::check_user_exists_by_email(&pool, NONEXISTENT_EMAIL)
                .await
                .unwrap();
        assert!(!exists.unwrap());

        Ok(())
    }

    #[sqlx::test(fixtures(path = "../../fixtures", scripts("users")))]
    async fn test_check_user_exists_by_nonexistent_account_id(
        pool: PgPool,
    ) -> sqlx::Result<()> {
        let exists = BwAccount::check_user_exists_by_account_id(
            &pool,
            &NONEXISTENT_ACCOUNT_ID,
        )
        .await
        .unwrap();
        assert!(!exists.unwrap());

        Ok(())
    }

    #[sqlx::test(fixtures(path = "../../fixtures", scripts("users")))]
    async fn test_check_user_active_by_nonexistent_account_id(
        pool: PgPool,
    ) -> sqlx::Result<()> {
        let is_active = BwAccount::check_user_active_by_account_id(
            &pool,
            NONEXISTENT_ACCOUNT_ID,
        )
        .await
        .unwrap();
        assert!(!is_active.unwrap()); // Assuming the account is inactive

        Ok(())
    }

    #[sqlx::test(fixtures(path = "../../fixtures", scripts("users")))]
    async fn test_update_last_login_for_nonexistent_account(
        pool: PgPool,
    ) -> sqlx::Result<()> {
        let rows_affected =
            BwAccount::update_last_login(&pool, NONEXISTENT_ACCOUNT_ID)
                .await
                .unwrap();
        assert_eq!(rows_affected, 0);

        Ok(())
    }

    #[sqlx::test(fixtures(path = "../../fixtures", scripts("users")))]
    async fn test_update_email_verified_at_for_nonexistent_account(
        pool: PgPool,
    ) -> sqlx::Result<()> {
        let rows_affected =
            BwAccount::update_email_verified_at(&pool, NONEXISTENT_ACCOUNT_ID)
                .await
                .unwrap();
        assert_eq!(rows_affected, 0);

        Ok(())
    }

    #[sqlx::test(fixtures(path = "../../fixtures", scripts("users")))]
    async fn test_update_password_for_nonexistent_account(
        pool: PgPool,
    ) -> sqlx::Result<()> {
        let new_password = ResetPasswordSchema {
            account_id: NONEXISTENT_ACCOUNT_ID,
            password: "new_password".to_string(),
        };
        let rows_affected =
            BwAccount::update_password_by_account_id(&pool, &new_password)
                .await
                .unwrap();
        assert_eq!(rows_affected, 0);

        Ok(())
    }
}
