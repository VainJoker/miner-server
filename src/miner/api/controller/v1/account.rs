use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};

use crate::{
    library::{
        crypto,
        error::{AppError::AuthError, AppResult, AuthInnerError},
        mailor::Email,
    },
    miner::{
        bootstrap::{
            constants::{self, MQ_SEND_EMAIL_QUEUE},
            AppState,
        },
        entity::{
            account::{
                ActiveAccountRequest, LoginResponse, ResetPasswordRequest,
            },
            claims::{Claims, RefreshTokenSchema, TokenType},
            common::SuccessResponse,
        },
        service::token_generator,
    },
    models::bw_account,
};

pub async fn register_user_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<bw_account::RegisterUserSchema>,
) -> AppResult<impl IntoResponse> {
    if bw_account::BwAccount::check_user_exists_by_email(
        state.get_db(),
        &body.email,
    )
    .await?
    .unwrap_or(true)
    {
        return Err(AuthError(AuthInnerError::UserAlreadyExists));
    }

    let hashed_password = crypto::hash_password(body.password.as_bytes())?;
    let new_bw_account = bw_account::CreateBwAccountSchema {
        name: body.name,
        email: body.email,
        password: hashed_password,
    };

    let user = bw_account::BwAccount::register_account(
        state.get_db(),
        &new_bw_account,
    )
    .await?;

    Ok(SuccessResponse {
        msg: "success",
        data: Some(Json(user)),
    })
}

pub async fn login_user_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<bw_account::LoginUserSchema>,
) -> AppResult<impl IntoResponse> {
    let users = bw_account::BwAccount::fetch_user_by_email_or_name(
        state.get_db(),
        &body.email_or_name,
    )
    .await?;
    if users.is_empty() {
        return Err(AuthError(AuthInnerError::WrongCredentials));
    }
    for user in users {
        if crypto::verify_password(&user.password, &body.password)? {
            let token =
                token_generator::generate_tokens_for_user(&user).await?;
            let affected = bw_account::BwAccount::update_last_login(
                state.get_db(),
                user.account_id,
            )
            .await?;
            if affected != 1 {
                tracing::error!(
                    "Failed to update last login time for user: {}",
                    user.account_id
                );
            }
            return Ok(SuccessResponse {
                msg: "Tokens generated successfully",
                data: Some(Json(LoginResponse::new(token, user))),
            });
        }
    }
    Err(AuthError(AuthInnerError::WrongCredentials))
}

pub async fn refresh_token_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RefreshTokenSchema>,
) -> AppResult<impl IntoResponse> {
    let claims = Claims::parse_token(&body.refresh_token, TokenType::REFRESH)?;

    let user = bw_account::BwAccount::fetch_user_by_account_id(
        state.get_db(),
        claims.uid,
    )
    .await?
    .ok_or(AuthError(AuthInnerError::WrongCredentials))?;

    let token = token_generator::generate_tokens_for_user(&user).await?;

    Ok(SuccessResponse {
        msg: "Tokens refreshed successfully",
        data: Some(Json(token)),
    })
}

pub async fn get_me_handler(
    State(state): State<Arc<AppState>>,
    claims: Claims,
) -> AppResult<impl IntoResponse> {
    let user = bw_account::BwAccount::fetch_user_by_email(
        state.get_db(),
        &claims.email,
    )
    .await?;
    Ok(SuccessResponse {
        msg: "success",
        data: Some(Json(user)),
    })
}

pub async fn send_active_account_email_handler(
    State(state): State<Arc<AppState>>,
    claims: Claims,
) -> AppResult<impl IntoResponse> {
    let active_code = crypto::random_words(6);
    let body = format!("Active Code: {}", active_code);

    state
        .redis
        .set_ex(
            &format!("{}_{}", claims.uid, constants::REDIS_ACTIVE_ACCOUNT_KEY),
            &active_code,
            60 * 5,
        )
        .await?;

    let email = Email::new(&claims.email, "Active your account", &body);
    let email_json = serde_json::to_string(&email).map_err(|e| {
        anyhow::anyhow!("Error occurred while sending email: {}", e)
    })?;
    state
        .get_mq()?
        .basic_send(MQ_SEND_EMAIL_QUEUE, &email_json)
        .await?;

    Ok(SuccessResponse {
        msg: "success",
        data: None::<()>,
    })
}

pub async fn send_reset_password_email_handler(
    State(state): State<Arc<AppState>>,
    claims: Claims,
) -> AppResult<impl IntoResponse> {
    let reset_password_code = crypto::random_words(6);
    let body = format!("ResetPassword Code: {}", reset_password_code);

    state
        .redis
        .set_ex(
            &format!("{}_{}", claims.uid, constants::REDIS_RESET_PASSWORD_KEY),
            &reset_password_code,
            60 * 5,
        )
        .await?;

    let email = Email::new(&claims.email, "Reset Password", &body);
    let email_json = serde_json::to_string(&email).map_err(|e| {
        anyhow::anyhow!("Error occurred while sending email: {}", e)
    })?;
    state
        .get_mq()?
        .basic_send(MQ_SEND_EMAIL_QUEUE, &email_json)
        .await?;

    Ok(SuccessResponse {
        msg: "success",
        data: None::<()>,
    })
}

pub async fn verify_active_account_code_handler(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    Json(body): Json<ActiveAccountRequest>,
) -> AppResult<impl IntoResponse> {
    let key = format!("{}_{}", claims.uid, constants::REDIS_ACTIVE_ACCOUNT_KEY);

    if let Some(active_code_stored) = state.redis.get(&key).await? {
        if active_code_stored == body.code {
            bw_account::BwAccount::update_email_verified_at(
                state.get_db(),
                claims.uid,
            )
            .await?;
            state.redis.del(&key).await?;
        } else {
            return Err(AuthError(AuthInnerError::WrongCode));
        }
    }

    let user = bw_account::BwAccount::fetch_user_by_account_id(
        state.get_db(),
        claims.uid,
    )
    .await?
    .ok_or(AuthError(AuthInnerError::WrongCredentials))?;

    let token = token_generator::generate_tokens_for_user(&user).await?;

    state.redis.del(&key).await?;

    Ok(SuccessResponse {
        msg: "success",
        data: Some(Json(token)),
    })
}

pub async fn change_password_handler(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    Json(body): Json<ResetPasswordRequest>,
) -> AppResult<impl IntoResponse> {
    let key = format!("{}_{}", claims.uid, constants::REDIS_RESET_PASSWORD_KEY);

    if let Some(active_code_stored) = state.redis.get(&key).await? {
        if active_code_stored == body.code {
            bw_account::BwAccount::update_email_verified_at(
                state.get_db(),
                claims.uid,
            )
            .await?;
            state.redis.del(&key).await?;
        } else {
            return Err(AuthError(AuthInnerError::WrongCode));
        }
    }

    let reset_password = bw_account::ResetPasswordSchema {
        account_id: claims.uid,
        password: crypto::hash_password(body.password.as_bytes())?,
    };
    bw_account::BwAccount::update_password_by_account_id(
        state.get_db(),
        &reset_password,
    )
    .await?;

    state.redis.del(&key).await?;

    Ok(SuccessResponse {
        msg: "success",
        data: None::<()>,
    })
}
