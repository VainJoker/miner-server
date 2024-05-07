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
            let token = Claims::generate_token(&user.email.to_string())?;

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

    let token = Claims::generate_token(&user.email.to_string())?;

    Ok(SuccessResponse {
        msg: "Tokens refreshed successfully",
        data: Some(Json(token)),
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

pub async fn send_email_handler(
    State(state): State<Arc<AppState>>,
    claims: Claims,
) -> AppResult<impl IntoResponse> {
    let active_code = crypto::random_words(6);
    let body = format!("Active Code: {}", active_code);
    let email = Email::new(&claims.uid, "Active your account", &body);
    let email_json = serde_json::to_string(&email).unwrap();
    state
        .get_mq()?
        .basic_send(SEND_EMAIL_QUEUE, &email_json)
        .await?;

    Ok(SuccessResponse {
        msg: "success",
        data: None::<()>,
    })
}
