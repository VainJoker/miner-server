use serde::{Deserialize, Serialize};

use crate::{
    miner::service::jwt::TokenSchema,
    models::{
        account::BwAccount,
        types::{Currency, Language},
    },
};

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub tokens: TokenSchema,
    pub name: String,
    pub email: String,
    pub local_currency: Currency,
    pub system_lang: Language,
}

impl LoginResponse {
    pub fn new(tokens: TokenSchema, user: BwAccount) -> Self {
        Self {
            tokens,
            name: user.name,
            email: user.email,
            local_currency: user.local_currency,
            system_lang: user.system_lang,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RegisterUserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserRequest {
    pub email_or_name: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CodeType {
    ActiveAccount,
    ResetPassword,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActiveAccountRequest {
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResetPasswordRequest {
    pub code: String,
    pub password: String,
}
