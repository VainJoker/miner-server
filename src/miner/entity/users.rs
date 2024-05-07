use serde::Serialize;

use super::claims::TokenSchema;
use crate::models::{
    bw_account::BwAccount,
    types::{Currency, Language},
};

#[derive(Debug, Serialize)]
pub struct UserSchema {
    pub token: TokenSchema,
    pub name: String,
    pub email: String,
    pub local_currency: Currency,
    pub system_lang: Language,
}

impl UserSchema {
    pub fn new(token: TokenSchema, user: BwAccount) -> Self {
        Self {
            token,
            name: user.name,
            email: user.email,
            local_currency: user.local_currency,
            system_lang: user.system_lang,
        }
    }
}
