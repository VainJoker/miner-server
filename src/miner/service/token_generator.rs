use crate::{
    library::error::{AppError, AppResult, AuthInnerError},
    miner::entity::claims::{Claims, TokenSchema, TokenType, UserInfo},
    models::{bw_account::BwAccount, types::AccountStatus},
};

pub async fn generate_tokens_for_user(
    user: &BwAccount,
) -> AppResult<TokenSchema> {
    let token_type = match user.status {
        AccountStatus::Active => TokenType::ACCESS,
        AccountStatus::Inactive => TokenType::BASIC,
        AccountStatus::Suspend => {
            return Err(AppError::AuthError(AuthInnerError::AccountSuspended));
        }
    };

    let user_info = UserInfo {
        uid: user.account_id,
        email: user.email.clone(),
    };
    let token = Claims::generate_tokens(&user_info, token_type)?;

    Ok(token)
}
