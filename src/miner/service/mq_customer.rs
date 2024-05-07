use std::sync::Arc;

use crate::{
    library::{error::AppResult, mailor::Email, mqer::Subscriber},
    miner::bootstrap::{
        constants::{SEND_EMAIL_QUEUE, SEND_EMAIL_TAG},
        AppState,
    },
};

pub struct MqCustomer {}

impl MqCustomer {
    // TODO: Stop when bootstrap receive C-c
    pub async fn serve(state: Arc<AppState>) {
        let customer = MqCustomer {};
        match customer.email_sender(state.clone()).await {
            Ok(()) => {}
            Err(e) => {
                tracing::error!("Email Sender Broken: {}", e)
            }
        };
    }
    pub async fn email_sender(&self, state: Arc<AppState>) -> AppResult<()> {
        tracing::debug!("customer started");
        let mqer = state.get_mq()?;
        let func = |message: String| {
            let result = serde_json::from_str::<Email>(&message)
                .map_err(|e| {
                    tracing::error!("Failed to parse email from message: {}", e)
                })
                .and_then(|email| {
                    email.sync_send_text().map_err(|e| {
                        tracing::error!("Failed to send email: {}", e)
                    })
                });
            if result.is_ok() {
                tracing::debug!("received:{}", message)
            }
        };
        let delegate = Subscriber::new(func);
        Ok(mqer
            .basic_receive(SEND_EMAIL_QUEUE, SEND_EMAIL_TAG, delegate)
            .await?)
    }
}
