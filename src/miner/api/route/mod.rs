use std::{sync::Arc, time::Duration};

use axum::{
    middleware::{from_fn, from_fn_with_state},
    routing::post,
    Router,
};
use tower_http::timeout::TimeoutLayer;

use super::{
    controller::handler_404,
    middleware::{auth, cors, log, req_id},
};
use crate::miner::{
    api::controller::v1::account::{
        get_me_handler, login_user_handler, register_user_handler,
        send_email_handler,
    },
    bootstrap::AppState,
};

pub fn init(inpay_state: Arc<AppState>) -> Router {
    // 开放
    let open = Router::new()
        .route("/auth/login", post(login_user_handler))
        .route("/auth/register", post(register_user_handler));

    // 需授权
    let auth = Router::new()
        .route("/users/me", post(get_me_handler))
        .route("/users/send_email", post(send_email_handler))
        .route_layer(from_fn_with_state(inpay_state.clone(), auth::handle))
        .with_state(inpay_state.clone());

    Router::new()
        .nest("/api/v1", open.merge(auth))
        .fallback(handler_404)
        .with_state(inpay_state)
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .layer(from_fn(log::handle))
        .layer(from_fn(cors::handle))
        .layer(from_fn(req_id::handle))
}
