use std::sync::Arc;

use crate::{library::mqer::Subscriber, miner::bootstrap::AppState};

pub struct MqCustomer {}

impl MqCustomer {
    pub async fn serve(state: Arc<AppState>) {
        let customer = MqCustomer {};
        customer.email_sender(state.clone()).await;
    }
    pub async fn email_sender(&self, state: Arc<AppState>) {
        tracing::debug!("mq customer");
        let mqer = state.get_mq().unwrap();
        let func = |message: String| {
            tracing::debug!("mq customer:{}", message);
            eprintln!("{message}");
        };
        let delegate = Subscriber::new(func);
        mqer.basic_receive("inpay.dev.queue", "inpay.dev.tag", delegate)
            .await
            .unwrap();
    }
}
