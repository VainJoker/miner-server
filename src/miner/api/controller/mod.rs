use axum::{http::StatusCode, response::IntoResponse};

pub mod v1;

#[allow(clippy::unused_async)]
pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}
