use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};

use crate::{
    library::{
        crypto,
        error::{AppError::AuthError, AppResult, AuthInnerError},
        mailor::Email,
    },
    miner::{
        bootstrap::{constants::SEND_EMAIL_QUEUE, AppState},
        entity::{
            claims::{Claims, RefreshTokenSchema},
            common::SuccessResponse,
            users::UserSchema,
        },
    },
    models::{bw_account, types::AccountStatus},
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
            let status = match user.status {
                AccountStatus::Active => true,
                AccountStatus::Inactive => false,
                AccountStatus::Suspend => {
                    return Err(AuthError(AuthInnerError::AccountSuspended));
                }
            };
            let token =
                Claims::generate_token(&user.email.to_string(), status)?;
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
                data: Some(Json(UserSchema::new(token, user))),
            });
        }
    }
    Err(AuthError(AuthInnerError::WrongCredentials))
}

pub async fn refresh_token_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RefreshTokenSchema>,
) -> AppResult<impl IntoResponse> {
    let claims = Claims::parse_refresh_token(&body.refresh_token)?;

    let user = bw_account::BwAccount::fetch_user_by_account_id(
        state.get_db(),
        (claims.uid)
            .parse::<i64>()
            .map_err(|_| AuthError(AuthInnerError::WrongCredentials))?,
    )
    .await?
    .ok_or(AuthError(AuthInnerError::WrongCredentials))?;

    let status = match user.status {
        AccountStatus::Active => true,
        AccountStatus::Inactive => false,
        AccountStatus::Suspend => {
            return Err(AuthError(AuthInnerError::AccountSuspended));
        }
    };
    let token = Claims::generate_token(&user.email.to_string(), status)?;
    let token_response = token.into_response();

    Ok(SuccessResponse {
        msg: "Tokens refreshed successfully",
        data: Some(Json(token_response)),
    })
}

pub async fn get_me_handler(
    State(state): State<Arc<AppState>>,
    claims: Claims,
) -> AppResult<impl IntoResponse> {
    let user =
        bw_account::BwAccount::fetch_user_by_email(state.get_db(), &claims.uid)
            .await?;
    Ok(SuccessResponse {
        msg: "success",
        data: Some(Json(user)),
    })
}

// TODO: change this to service nmaed send_email_service then change the name of
// the function to send_verify_email_handler and add a new function named
// send_reset_password_email_handler
pub async fn send_email_handler(
    State(state): State<Arc<AppState>>,
    claims: Claims,
) -> AppResult<impl IntoResponse> {
    let active_code = crypto::random_words(6);
    let body = format!("Active Code: {}", active_code);

    state
        .redis
        .set_ex(&format!("{}_active_code", claims.uid), &active_code, 60)
        .await?;

    let email = Email::new(&claims.uid, "Active your account", &body);
    let email_json = serde_json::to_string(&email).map_err(|e| {
        anyhow::anyhow!("Error occurred while sending email: {}", e)
    })?;
    state
        .get_mq()?
        .basic_send(SEND_EMAIL_QUEUE, &email_json)
        .await?;

    Ok(SuccessResponse {
        msg: "success",
        data: None::<()>,
    })
}

pub async fn verify_email_handler(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    Json(body): Json<String>,
) -> AppResult<impl IntoResponse> {
    let uid = claims.uid.parse::<i64>().map_err(|e| {
        anyhow::anyhow!("Error occurs while verify email: {}", e)
    })?;
    if let Some(active_code_stored) = state
        .redis
        .get(&format!("{}_active_code", claims.uid))
        .await?
    {
        if active_code_stored == body {
            bw_account::BwAccount::update_email_verified_at(
                state.get_db(),
                uid,
            )
            .await?;
            state
                .redis
                .del(&format!("{}_active_code", claims.uid))
                .await?;
        } else {
            return Err(AuthError(AuthInnerError::WrongCode));
        }
    }

    Ok(SuccessResponse {
        msg: "success",
        data: None::<()>,
    })
}

// TODO: add reset password handler

// pub async fn change_password_handler(
//     State(state): State<Arc<AppState>>,
//     claims: Claims,
//     Json(body): Json<String>,
// )
