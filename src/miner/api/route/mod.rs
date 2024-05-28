use std::{sync::Arc, time::Duration};

use axum::{
    middleware::{from_fn, from_fn_with_state},
    routing::post,
    Router,
};
use tower_http::timeout::TimeoutLayer;

use super::{
    controller::{
        handler_404,
        v1::{account::{
            change_password_handler, refresh_token_handler,
            send_reset_password_email_handler,
            verify_active_account_code_handler,
        }, operate::operate_handler},
    },
    middleware::{auth, basic_auth, cors, log, req_id},
};
use crate::miner::{
    api::controller::v1::{
        account::{
            get_me_handler, login_user_handler, register_user_handler,
            send_active_account_email_handler,
        },
        group::{
            create_group_handler, delete_group_handler,
            get_groups_by_ids_handler, get_groups_handler,
            update_group_handler,
        },
    },
    bootstrap::AppState,
};

pub fn init(miner_state: Arc<AppState>) -> Router {
    let open = Router::new()
        .route("/auth/login", post(login_user_handler))
        .route("/auth/register", post(register_user_handler))
        .route("/users/refresh_token", post(refresh_token_handler));

    let basic = Router::new()
        .route(
            "/users/send_active",
            post(send_active_account_email_handler),
        )
        .route(
            "/users/verify_active",
            post(verify_active_account_code_handler),
        )
        .layer(from_fn(basic_auth::handle));

    let auth = Router::new()
        .route("/users/get_me", post(get_me_handler))
        .route(
            "/users/send_reset_password",
            post(send_reset_password_email_handler),
        )
        .route(
            "/users/verify_reset_password",
            post(change_password_handler),
        )
        .route("/groups/list", post(get_groups_handler))
        .route("/groups/create", post(create_group_handler))
        .route("/groups/update", post(update_group_handler))
        .route("/groups/delete", post(delete_group_handler))
        .route("/groups/ids", post(get_groups_by_ids_handler))

        .route("/operate/do", post(operate_handler))
        .route_layer(from_fn_with_state(miner_state.clone(), auth::handle))
        .with_state(miner_state.clone());

    Router::new()
        .nest("/api/v1", open.merge(basic).merge(auth))
        .fallback(handler_404)
        .with_state(miner_state)
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .layer(from_fn(log::handle))
        .layer(from_fn(cors::handle))
        .layer(from_fn(req_id::handle))
}
