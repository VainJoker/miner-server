use std::sync::Arc;
use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use crate::library::error::{AppResult, AuthInnerError};
use crate::miner::bootstrap::AppState;
use crate::miner::entity::common::SuccessResponse;
use crate::miner::entity::group::CreateGroupRequest;
use crate::miner::service::jwt::Claims;
use crate::models::bw_account::{BwAccount, CreateBwAccountSchema, RegisterUserRequest};

// pub async fn create_group_handler(
//     State(state): State<Arc<AppState>>,
//     Json(body): Json<CreateGroupRequest>,
//     claims: Claims
// ) -> AppResult<impl IntoResponse> {
//
//
//     Ok(SuccessResponse {
//         msg: "success",
//         data: Some(Json(user)),
//     })
// }
