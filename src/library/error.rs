use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

pub type InnerResult<T> = Result<T, AppInnerError>;

#[derive(Error, Debug)]
pub enum AppInnerError {
    #[error("Database error: ")]
    DataBaseError(#[from] sqlx::Error),
    #[error(transparent)]
    RedisError(#[from] RedisorError),
    #[error("RabbitMq error: ")]
    MQError(#[from] MqerError),
    #[error("Email error: ")]
    EmailError(#[from] lettre::transport::smtp::Error),
    #[error("Internal server error")]
    Unknown(String),
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum RedisorError {
    #[error("Redis connection error: ")]
    PoolError(#[from] deadpool_redis::PoolError),
    #[error("Redis execution error: ")]
    ExeError(#[from] redis::RedisError),
}

#[derive(Error, Debug)]
pub enum MqerError {
    #[error("Mq connection error: ")]
    PoolError(#[from] deadpool_lapin::PoolError),
    #[error("Mq execution error: ")]
    ExeError(#[from] deadpool_lapin::lapin::Error),
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Unknown error `{0}`")]
    Unknown(String),

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),

    #[error("System error `{0}`")]
    ErrSystem(String),

    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumFormRejection(#[from] axum::extract::rejection::FormRejection),

    #[error(transparent)]
    InnerError(#[from] AppInnerError),

    #[error(transparent)]
    AuthError(#[from] AuthInnerError),
}

#[derive(Error, Debug)]
pub enum AuthInnerError {
    #[error("UserAlreadyExists")]
    UserAlreadyExists,
    #[error("WrongCredentials")]
    WrongCredentials,
    #[error("MissingCredentials")]
    MissingCredentials,
    #[error("TokenCreation")]
    TokenCreation,
    #[error("InvalidToken")]
    InvalidToken,
}

impl AppError {
    pub fn select_status_code(app_error: &Self) -> (StatusCode, u32) {
        match app_error {
            Self::ValidationError(_) => {
                (StatusCode::UNPROCESSABLE_ENTITY, 20001)
            }
            Self::AuthError(e) => match e {
                AuthInnerError::WrongCredentials => {
                    (StatusCode::UNAUTHORIZED, 10001)
                }
                AuthInnerError::TokenCreation => (StatusCode::FORBIDDEN, 10002),
                AuthInnerError::InvalidToken => {
                    (StatusCode::UNAUTHORIZED, 10003)
                }
                AuthInnerError::UserAlreadyExists => {
                    (StatusCode::CONFLICT, 10004)
                }
                AuthInnerError::MissingCredentials => {
                    (StatusCode::UNAUTHORIZED, 10005)
                }
            },
            _ => (StatusCode::BAD_REQUEST, 99999),
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code) = Self::select_status_code(&self);
        let body = axum::Json(serde_json::json!({
            "code": code,
            "msg": format!("{self}")
        }));
        (status, body).into_response()
    }
}
