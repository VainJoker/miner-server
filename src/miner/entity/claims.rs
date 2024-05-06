// use anyhow::anyhow;
// use axum::{
//     async_trait, extract::FromRequestParts, http::request::Parts,
//     RequestPartsExt,
// };
// use axum_extra::{
//     headers::{authorization::Bearer, Authorization},
//     TypedHeader,
// };
// use jsonwebtoken::{
//     decode, encode, DecodingKey, EncodingKey, Header, Validation,
// };
// use serde::{Deserialize, Serialize};
//
// use crate::library::{
//     cfg,
//     error::{AppError, AppResult},
// };
// use crate::library::error::AuthInnerError;
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct Claims {
//     pub uid: String,
//     pub iat: usize,
//     pub exp: usize,
// }
//
// #[derive(Debug, Serialize)]
// pub struct TokenSchema {
//     pub refresh_token: String,
//     pub access_token: String,
// }
//
// pub struct RefreshTokenSchema {
//     pub refresh_token: String,
// }
//
// #[async_trait]
// impl<S> FromRequestParts<S> for Claims
// where
//     S: Send + Sync,
// {
//     type Rejection = AppError;
//
//     async fn from_request_parts(
//         parts: &mut Parts,
//         _state: &S,
//     ) -> AppResult<Self> {
//         // Extract the token from the authorization header
//         let TypedHeader(Authorization(bearer)) = parts
//             .extract::<TypedHeader<Authorization<Bearer>>>()
//             .await
//             .map_err(
//                 AppError::AuthError(AuthInnerError::InvalidToken)
//             )?;
//         // Decode the user data
//         let claims = Self::parse_access_token(bearer.token())?;
//
//         Ok(claims)
//     }
// }
//
// impl Claims {
//     pub fn generate_access_token(credential: &str) -> AppResult<String> {
//         let now = chrono::Utc::now();
//         let duration = cfg::config().inpay.access_secret_expiration;
//         let claims = Self {
//             uid: credential.to_string(),
//             exp: (now +
// chrono::Duration::seconds(duration.into())).timestamp()                 as
// usize,             iat: now.timestamp() as usize,
//         };
//
//         let token = encode(
//             &Header::default(),
//             &claims,
//             &EncodingKey::from_secret(
//                 cfg::config().inpay.access_secret.as_ref(),
//             ),
//         )
//         .map_err(|e| {
//             AppError::AuthError(AuthInnerError::TokenCreation)
//         })?;
//         Ok(token)
//     }
//
//     pub fn parse_access_token(token: &str) -> AppResult<Self> {
//         let token_data = decode::<Self>(
//             token,
//             &DecodingKey::from_secret(
//                 cfg::config().inpay.access_secret.as_ref(),
//             ),
//             &Validation::default(),
//         )
//         .map_err(|e| {
//             AppError::AuthError(AuthInnerError::InvalidToken)
//         })?;
//         Ok(token_data.claims)
//     }
//
//     pub fn generate_refresh_token(credential: &str) -> AppResult<String> {
//         let now = chrono::Utc::now();
//         let duration = cfg::config().inpay.refresh_secret_expiration;
//         let claims = Self {
//             uid: credential.to_string(),
//             exp: (now +
// chrono::Duration::seconds(duration.into())).timestamp()                 as
// usize,             iat: now.timestamp() as usize,
//         };
//         let token = encode(
//             &Header::default(),
//             &claims,
//             &EncodingKey::from_secret(
//                 cfg::config().inpay.access_secret.as_ref(),
//             ),
//         )
//         .map_err(|e| {
//             AppError::AuthError(AuthInnerError::TokenCreation)
//         })?;
//         Ok(token)
//     }
//
//     pub fn parse_refresh_token(token: &str) -> AppResult<Self> {
//         let token_data = decode::<Self>(
//             token,
//             &DecodingKey::from_secret(
//                 cfg::config().inpay.access_secret.as_ref(),
//             ),
//             &Validation::default(),
//         )
//         .map_err(|e| {
//             AppError::AuthError(AuthInnerError::InvalidToken)
//         })?;
//         Ok(token_data.claims)
//     }
//
//     pub fn generate_token(credential: &str) -> AppResult<TokenSchema> {
//         let access_token = Self::generate_access_token(credential)?;
//         let refresh_token = Self::generate_refresh_token(credential)?;
//
//         Ok(TokenSchema {
//             refresh_token,
//             access_token,
//         })
//     }
// }

use axum::{
    async_trait, extract::FromRequestParts, http::request::Parts,
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{
    decode, encode, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};

use crate::library::{
    cfg,
    error::{AppError, AppResult, AuthInnerError},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub uid: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Serialize)]
pub struct TokenSchema {
    pub refresh_token: String,
    pub access_token: String,
}

pub struct RefreshTokenSchema {
    pub refresh_token: String,
}

// fn handle_auth_error(detail: AuthInnerError) -> AppError {
//     AppError::AuthError(detail)
// }

fn get_access_secret<'a>() -> &'a [u8] {
    cfg::config().inpay.access_secret.as_ref()
}

fn get_access_secret_expiration() -> i64 {
    cfg::config().inpay.access_secret_expiration.into()
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> AppResult<Self> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AppError::AuthError(AuthInnerError::InvalidToken))?;

        let claims = Self::parse_access_token(bearer.token())?;

        Ok(claims)
    }
}

impl Claims {
    pub fn generate_access_token(credential: &str) -> AppResult<String> {
        let now = chrono::Utc::now();
        let duration = get_access_secret_expiration();
        let claims = Self {
            uid: credential.to_string(),
            exp: (now + chrono::Duration::seconds(duration)).timestamp()
                as usize,
            iat: now.timestamp() as usize,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(get_access_secret()),
        )
        .map_err(|_| AppError::AuthError(AuthInnerError::TokenCreation))?;

        Ok(token)
    }

    pub fn parse_access_token(token: &str) -> AppResult<Self> {
        let token_data = decode::<Self>(
            token,
            &DecodingKey::from_secret(get_access_secret()),
            &Validation::default(),
        )
        .map_err(|_| AppError::AuthError(AuthInnerError::InvalidToken))?;

        Ok(token_data.claims)
    }

    pub fn generate_refresh_token(credential: &str) -> AppResult<String> {
        let now = chrono::Utc::now();
        let duration = get_access_secret_expiration();
        let claims = Self {
            uid: credential.to_string(),
            exp: (now + chrono::Duration::seconds(duration)).timestamp()
                as usize,
            iat: now.timestamp() as usize,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(get_access_secret()),
        )
        .map_err(|_| AppError::AuthError(AuthInnerError::TokenCreation))?;

        Ok(token)
    }

    pub fn parse_refresh_token(token: &str) -> AppResult<Self> {
        let token_data = decode::<Self>(
            token,
            &DecodingKey::from_secret(get_access_secret()),
            &Validation::default(),
        )
        .map_err(|_| AppError::AuthError(AuthInnerError::InvalidToken))?;

        Ok(token_data.claims)
    }

    pub fn generate_token(credential: &str) -> AppResult<TokenSchema> {
        let access_token = Self::generate_access_token(credential)?;
        let refresh_token = Self::generate_refresh_token(credential)?;

        Ok(TokenSchema {
            refresh_token,
            access_token,
        })
    }
}
