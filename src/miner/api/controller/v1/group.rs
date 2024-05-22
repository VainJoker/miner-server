use std::sync::Arc;
use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use crate::library::error::{ApiInnerError, AppError, AppResult, AuthInnerError};
use crate::library::error::AppError::ApiError;
use crate::miner::bootstrap::AppState;
use crate::miner::entity::common::SuccessResponse;
use crate::miner::entity::group::CreateGroupRequest;
use crate::miner::service::jwt::Claims;
use crate::models::group::{BwGroup, CreateBwGroupSchema};


pub async fn create_group_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateGroupRequest>,
    claims: Claims
) -> AppResult<impl IntoResponse> {
    let new_group = CreateBwGroupSchema{
        account_id: claims.uid,
        name: body.name,
        remark: body.remark,
    };
    let group = BwGroup::create_bw_group(state.get_db(),&new_group).await.map_err(|_| ApiError(ApiInnerError::CreateGroupError))?;

    Ok(SuccessResponse {
        msg: "success",
        data: Some(Json(group)),
    })
}
