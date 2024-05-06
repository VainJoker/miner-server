use std::str::FromStr;

use serde::{Deserialize, Serialize};
use sqlx::{types::chrono::NaiveDateTime, PgPool};
use uuid::Uuid;

use crate::{
    entity::types::{Currency, Language},
    library::error::InnerResult,
};

#[allow(dead_code)]
#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
#[sqlx(rename_all = "lowercase")]
pub struct BwAccount {
    pub id: i32,
    pub account_id: Uuid,
    pub name: String,
    pub email: String,
    pub email_verified_at: Option<NaiveDateTime>,
    pub password: String,
    pub local_currency: Currency,
    pub system_lang: Language,
    pub registered_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct RegisterUserSchema {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserSchema {
    pub email_or_name: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateBwAccountSchema {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl BwAccount {
    pub async fn create(
        db: &PgPool,
        new_bw_account: &CreateBwAccountSchema,
    ) -> InnerResult<Self> {
        let map = sqlx::query_as!(
            Self,
            r#"
                INSERT INTO bw_account (name, email, password) VALUES ($1, $2, $3)
                RETURNING id,account_id,name,email,email_verified_at,password,
                    local_currency as "local_currency: _",system_lang as "system_lang: _",
                    registered_at, created_at,updated_at
            "#,
            new_bw_account.name,
            new_bw_account.email,
            new_bw_account.password
        );

        Ok(map.fetch_one(db).await?)
    }
    pub async fn read(db: &PgPool) -> InnerResult<Vec<Self>> {
        let map = sqlx::query_as!(
            Self,
            r#"SELECT id,account_id,name,email,email_verified_at,password,
            local_currency as "local_currency: _",system_lang as "system_lang: _",
            registered_at, created_at,updated_at FROM bw_account"#
        );

        Ok(map.fetch_all(db).await?)
    }
    // fn update(db: &PgPool) {
    //
    // }
    // fn delete(db: &PgPool) {
    //
    // }
    pub async fn check_user_exists_by_email(
        db: &PgPool,
        email: &str,
    ) -> InnerResult<Option<bool>> {
        let map = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM bw_account WHERE email = $1)",
            email.to_owned().to_ascii_lowercase()
        );
        Ok(map.fetch_one(db).await?)
    }

    pub async fn check_user_exists_by_account_id(
        db: &PgPool,
        account_id: &Uuid,
    ) -> InnerResult<Option<bool>> {
        let map = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM bw_account WHERE account_id = $1)",
            account_id
        );
        Ok(map.fetch_one(db).await?)
    }

    // TODO: what will happen if many user have the same name
    pub async fn fetch_user_by_email_or_name(
        db: &PgPool,
        email_or_name: &str,
    ) -> InnerResult<Option<Self>> {
        let map = sqlx::query_as!(
            Self,
            r#"SELECT id,account_id,name,email,email_verified_at,password,
            local_currency as "local_currency: _",system_lang as "system_lang: _",
            registered_at, created_at,updated_at
            FROM bw_account WHERE name = $1 or email = $1"#,
            email_or_name
        );
        Ok(map.fetch_optional(db).await?)
    }

    pub async fn fetch_user_by_account_id(
        db: &PgPool,
        account_id: &str,
    ) -> InnerResult<Option<Self>> {
        let account_id = Uuid::from_str(account_id).unwrap();
        let map = sqlx::query_as!(
            Self,
            r#"SELECT id,account_id,name,email,email_verified_at,password,
            local_currency as "local_currency: _",system_lang as "system_lang: _",
            registered_at, created_at,updated_at
            FROM bw_account WHERE account_id = $1"#,
            account_id
        );
        Ok(map.fetch_optional(db).await?)
    }
}
