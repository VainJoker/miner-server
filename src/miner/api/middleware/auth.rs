use axum::{
    extract::Request, http::header::AUTHORIZATION, middleware::Next,
    response::Response,
};

use crate::{
    library::error::{AppError::AuthError, AppResult, AuthInnerError},
    miner::entity::claims::{Claims, TokenType},
};

pub async fn handle(request: Request, next: Next) -> AppResult<Response> {
    let token = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| auth_value.strip_prefix("Bearer "))
        .ok_or(AuthError(AuthInnerError::InvalidToken))?;

    Claims::parse_token(token, TokenType::ACCESS)?;

    Ok(next.run(request).await)
}
