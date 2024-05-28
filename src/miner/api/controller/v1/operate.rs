use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};

use crate::{
    library::error::AppResult,
    miner::{
        bootstrap::AppState,
        entity::{common::SuccessResponse, operate::OperateRequest},
        service::jwt_service::Claims,
    },
};

#[axum::debug_handler]
pub async fn operate_handler(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    Json(body): Json<OperateRequest>,
) -> AppResult<impl IntoResponse> {
    Ok(SuccessResponse {
        msg: "success",
        data: Some(Json(body)),
    })
}
