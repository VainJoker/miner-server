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
//     let r_user_key = &format!("miner_users:{}",  claims.uid);
//
//     let mut redis = state.get_redis().await?;
//     let r_user_fields = redis.hkeys(r_user_key).await?;
//
//     let r_status_keys = r_user_fields
//         .iter()
//         .map(|k| format!("miner_status:{}", k))
//         .collect();
//
//     if r_status_keys.is_empty() {
//        todo!()
//     }
//
//     let r_status_value = redis.mget(&r_status_keys).await?;
//     let none_indices: Vec<usize> = r_status_value.iter().enumerate()
//         .filter_map(|(i, &item)| {
//             if item.is_none() {
//                 Some(i)
//             } else {
//                 None
//             }
//         })
//         .collect();
//     if none_indices.is_empty() {
//         todo!()
//     }
//     let none_machine_hash_keys = none_indices.iter().map(|i|{
//         r_status_keys[*i].to_owned()
//     }).collect();
//
//     let r_user_values = redis.hmget(r_user_key,
// &none_machine_hash_keys).await?;
//
//     let mut machines = Vec::new();
//
//     machines.extend(r_status_value);
//     machines.extend(r_user_values);
//     machines.retain(|m| m.is_some());
//
//     Ok(Json(machines))
// }
