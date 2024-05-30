// use std::sync::Arc;
//
// use axum::{extract::State, response::IntoResponse, Json};
//
// use crate::{
//     library::error::{
//         ApiInnerError, AppError, AppError::ApiError, AppInnerError,
// AppResult,     },
//     miner::{
//         bootstrap::AppState, entity::common::SuccessResponse,
//         service::jwt_service::Claims,
//     },
//     models::{group::BwGroup, machine::BwMachine},
// };
//
// pub async fn get_machines_handler(
//     State(state): State<Arc<AppState>>,
//     claims: Claims,
// ) -> AppResult<impl IntoResponse> {
//     let account_id = claims.uid;
//     let user_key = &format!("machine_users:{}", account_id);
//
//     let redis = state.get_redis();
//     let res = redis.get_hash_keys(user_key).await?;
//
//     // let machines_key = &format!("machines:{}", account_id);
//     // let redis = state.get_redis();
//     // let res = redis.get(machines_key).await?;
// }
