use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};

use crate::{
    library::error::{ApiInnerError, AppError::ApiError, AppResult},
    miner::{
        bootstrap::AppState,
        entity::{
            common::SuccessResponse,
            group::{
                CreateGroupRequest, DeleteBwGroupRequest, ReadBwGroupRequest,
                UpdateBwGroupRequest,
            },
        },
        service::jwt_service::Claims,
    },
    models::group::{
        BwGroup, CreateBwGroupSchema, DeleteBwGroupSchema, ReadBwGroupSchema,
        UpdateBwGroupSchema,
    },
};

pub async fn create_group_handler(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    Json(body): Json<CreateGroupRequest>,
) -> AppResult<impl IntoResponse> {
    let item = CreateBwGroupSchema {
        account_id: claims.uid,
        name: body.name,
        remark: body.remark,
    };
    let group = BwGroup::create_bw_group(state.get_db(), &item)
        .await
        .map_err(|_| ApiError(ApiInnerError::CreateGroupError))?;

    Ok(SuccessResponse {
        msg: "success",
        data: Some(Json(group)),
    })
}

pub async fn get_groups_handler(
    State(state): State<Arc<AppState>>,
    claims: Claims,
) -> AppResult<impl IntoResponse> {
    let group = BwGroup::fetch_group_by_account_id(state.get_db(), claims.uid)
        .await
        .map_err(|_| ApiError(ApiInnerError::GetGroupError))?;
    Ok(SuccessResponse {
        msg: "success",
        data: Some(Json(group)),
    })
}

pub async fn get_groups_by_ids_handler(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    Json(body): Json<ReadBwGroupRequest>,
) -> AppResult<impl IntoResponse> {
    let item = ReadBwGroupSchema {
        account_id: claims.uid,
        group_ids: body.group_ids,
    };

    let groups = BwGroup::fetch_group_info_by_ids(state.get_db(), &item)
        .await
        .map_err(|_| ApiError(ApiInnerError::GetGroupError))?;
    Ok(SuccessResponse {
        msg: "success",
        data: Some(Json(groups)),
    })
}

pub async fn delete_group_handler(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    Json(body): Json<DeleteBwGroupRequest>,
) -> AppResult<impl IntoResponse> {
    let item = DeleteBwGroupSchema {
        group_id: body.group_id,
        account_id: claims.uid,
    };
    let rows_affected =
        BwGroup::delete_group_by_group_id(state.get_db(), &item)
            .await
            .map_err(|_| ApiError(ApiInnerError::DeleteGroupError))?;
    if rows_affected != 0 {
        return Err(ApiError(ApiInnerError::DeleteGroupError));
    }
    Ok(SuccessResponse {
        msg: "success",
        data: None::<()>,
    })
}

pub async fn update_group_handler(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    Json(body): Json<UpdateBwGroupRequest>,
) -> AppResult<impl IntoResponse> {
    let item = UpdateBwGroupSchema {
        group_id: body.group_id,
        account_id: claims.uid,
        name: body.name,
        remark: body.remark,
    };
    let rows_affected =
        BwGroup::update_group_by_group_id(state.get_db(), &item)
            .await
            .map_err(|_| ApiError(ApiInnerError::UpdateGroupError))?;
    if rows_affected != 0 {
        return Err(ApiError(ApiInnerError::UpdateGroupError));
    }
    Ok(SuccessResponse {
        msg: "success",
        data: None::<()>,
    })
}
