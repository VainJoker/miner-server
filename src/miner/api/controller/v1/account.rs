use std::sync::Arc;

use anyhow::anyhow;
use argon2::{
    password_hash::SaltString, Argon2, PasswordHash, PasswordHasher,
    PasswordVerifier,
};
use axum::{extract::State, response::IntoResponse, Json};
use rand_core::OsRng;

use crate::{
    entity::bw_account,
    library::{
        error::{AppError, AppError::AuthError, AuthInnerError, MinerResult},
        mailor::Email,
    },
    miner::{
        bootstrap::AppState,
        entity::{
            claims::{Claims, RefreshTokenSchema},
            common::SuccessResponse,
            users::UserSchema,
        },
    },
};
use crate::miner::service;
use crate::miner::service::mq_customer::MqCustomer;

pub async fn register_user_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<bw_account::RegisterUserSchema>,
) -> MinerResult<impl IntoResponse> {
    if bw_account::BwAccount::check_user_exists_by_email(
        state.get_db(),
        &body.email,
    )
    .await?
    .unwrap_or(false)
    {
        return Err(AuthError(AuthInnerError::UserAlreadyExists));
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .map_err(|e| {
            AppError::Anyhow(anyhow!("Error while hashing password: {}", e))
        })
        .map(|hash| hash.to_string())?;
    let new_bw_account = bw_account::CreateBwAccountSchema {
        name: body.name,
        email: body.email,
        password: hashed_password,
    };

    let user =
        bw_account::BwAccount::create(state.get_db(), &new_bw_account).await?;

    Ok(SuccessResponse {
        msg: "success",
        data: Some(Json(user)),
    })
}

pub async fn login_user_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<bw_account::LoginUserSchema>,
) -> MinerResult<impl IntoResponse> {
    let user = bw_account::BwAccount::fetch_user_by_email_or_name(
        state.get_db(),
        &body.email_or_name,
    )
    .await?
    .ok_or(AuthError(AuthInnerError::WrongCredentials))?;

    let is_valid = match PasswordHash::new(&user.password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_or(false, |()| true),
        Err(_) => false,
    };

    if !is_valid {
        return Err(AuthError(AuthInnerError::WrongCredentials));
    }

    let token = Claims::generate_token(&user.account_id.to_string())?;

    Ok(SuccessResponse {
        msg: "Tokens generated successfully",
        data: Some(Json(UserSchema::new(token, user))),
    })
}

pub async fn refresh_token_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RefreshTokenSchema>,
) -> MinerResult<impl IntoResponse> {
    let claims = Claims::parse_refresh_token(&body.refresh_token)?;

    let user = bw_account::BwAccount::fetch_user_by_account_id(
        state.get_db(),
        &claims.uid,
    )
    .await?
    .ok_or(AuthError(AuthInnerError::WrongCredentials))?;

    let token = Claims::generate_token(&user.account_id.to_string())?;

    Ok(SuccessResponse {
        msg: "Tokens refreshed successfully",
        data: Some(Json(token)),
    })
}

pub async fn get_me_handler(
    State(state): State<Arc<AppState>>,
    claims: Claims,
) -> MinerResult<impl IntoResponse> {
    let user = bw_account::BwAccount::fetch_user_by_account_id(
        state.get_db(),
        &claims.uid,
    )
    .await?;
    Ok(SuccessResponse {
        msg: "success",
        data: Some(Json(user)),
    })
}

pub async fn send_email_handler(
    State(state): State<Arc<AppState>>,
) -> MinerResult<impl IntoResponse> {
    let body = format!("你的激活码是：{}", 1121);
    let email = Email::new("vainjoker@tuta.io", "激活账号", &body);
    let email_json = serde_json::to_string(&email).unwrap();
    state.get_mq().unwrap().basic_send("inpay.dev.queue", &email_json).await.unwrap();
    // let mq_customer = MqCustomer{mqer: state.get_mq()};
    // mq_customer.email_sender(&email).await;
    // Ok(axum::response::Redirect::to("/").into_response())

    Ok(SuccessResponse {
        msg: "success",
        data: Some(Json("11")),
    })
}
