use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};

use crate::{
    library::error::{
        ApiInnerError, AppError::ApiError, AppResult,
    },
    miner::{
        bootstrap::AppState,
        entity::{common::SuccessResponse, group::CreateGroupRequest},
        service::jwt::Claims,
    },
    models::group::{BwGroup, CreateBwGroupSchema},
};

pub async fn create_group_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateGroupRequest>,
    claims: Claims,
) -> AppResult<impl IntoResponse> {
    let new_group = CreateBwGroupSchema {
        account_id: claims.uid,
        name: body.name,
        remark: body.remark,
    };
    let group = BwGroup::create_bw_group(state.get_db(), &new_group)
        .await
        .map_err(|_| ApiError(ApiInnerError::CreateGroupError))?;

    Ok(SuccessResponse {
        msg: "success",
        data: Some(Json(group)),
    })
}
