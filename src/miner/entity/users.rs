use serde::Serialize;

use super::claims::{TokenSchema, TokenSchemaResponse};
use crate::models::{
    bw_account::BwAccount,
    types::{Currency, Language},
};

#[derive(Debug, Serialize)]
pub struct UserSchema {
    pub token: TokenSchemaResponse,
    pub name: String,
    pub email: String,
    pub local_currency: Currency,
    pub system_lang: Language,
}

impl UserSchema {
    pub fn new(token: TokenSchema, user: BwAccount) -> Self {
        let token_response = token.into_response();
        Self {
            token: token_response,
            name: user.name,
            email: user.email,
            local_currency: user.local_currency,
            system_lang: user.system_lang,
        }
    }
}
