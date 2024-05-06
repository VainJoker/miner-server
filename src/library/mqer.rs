use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, RwLock},
};

use deadpool_lapin::{
    lapin::{
        message::DeliveryResult,
        options::{
            BasicAckOptions, BasicConsumeOptions, BasicPublishOptions,
            ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions,
        },
        types::FieldTable,
        BasicProperties, ConsumerDelegate, ExchangeKind,
    },
    Object, Runtime,
};

use crate::library::{
    cfg,
    error::{InnerResult, MqerError},
};

pub type MQ = Object;
pub struct Mqer {
    pub pool: deadpool_lapin::Pool,
    pub running: Arc<RwLock<bool>>,
}

#[derive(Clone)]
pub struct Subscriber {
    pub func: Arc<Box<dyn Fn(String) + Send + Sync>>,
}

impl Subscriber {
    pub fn new<F>(func: F) -> Self
    where
        F: Fn(String) + Send + Sync + 'static,
    {
        Self {
            func: Arc::new(Box::new(func)),
        }
    }
}

impl ConsumerDelegate for Subscriber {
    fn on_new_delivery(
        &self,
        delivery: DeliveryResult,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let func = Arc::clone(&self.func);
        Box::pin(async move {
            if let Ok(Some(delivery)) = delivery {
                let message = String::from_utf8_lossy(&delivery.data);
                (func)(message.to_string());
                delivery.ack(BasicAckOptions::default()).await.unwrap();
            } else {
                tracing::error!("Failed to consume queue message");
            };
        })
    }
}

impl Mqer {
    pub fn init() -> Self {
        let cfg = cfg::config();
        let mq_url = cfg.inpay.mq_url.clone();

        let deadpool = deadpool_lapin::Config {
            url: Some(mq_url),
            ..Default::default()
        };
        match deadpool.create_pool(Some(Runtime::Tokio1)) {
            Ok(pool) => {
                tracing::info!("🚀 Connection to the mq is successful!");
                let running = Arc::new(RwLock::new(true));
                Self { pool, running }
            }
            Err(err) => {
                panic!("💥 Failed to connect to the redis: {err:?}");
            }
        }
    }

    pub async fn get_conn(&self) -> InnerResult<Object> {
        Ok(self.pool.get().await.map_err(MqerError::PoolError)?)
    }

    pub fn get_running(&self) -> bool {
        *self.running.read().unwrap()
    }


    pub async fn basic_send(
        &self,
        queue_name: &str,
        payload: &str,
    ) -> InnerResult<()> {
        let chan = self
            .get_conn()
            .await?
            .create_channel()
            .await
            .map_err(MqerError::ExeError)?;

        let queue = chan
            .queue_declare(
                queue_name,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .map_err(MqerError::ExeError)?;

        let payload = payload.as_bytes();

        chan.basic_publish(
            "",
            queue.name().as_str(),
            BasicPublishOptions::default(),
            payload,
            BasicProperties::default(),
        )
        .await
        .map_err(MqerError::ExeError)?
        .await
        .map_err(MqerError::ExeError)?;
        Ok(())
    }

    pub async fn basic_receive(
        &self,
        queue_name: &str,
        tag: &str,
        delegate: impl ConsumerDelegate + 'static,
    ) -> InnerResult<()> {
        let chan = self
            .get_conn()
            .await?
            .create_channel()
            .await
            .map_err(MqerError::ExeError)?;

        let queue = chan
            .queue_declare(
                queue_name,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .map_err(MqerError::ExeError)?;

        chan.basic_consume(
            queue.name().as_str(),
            tag,
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .map_err(MqerError::ExeError)?
        .set_delegate(delegate);

        while self.get_running() {
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        }
        Ok(())
    }

    pub async fn topic_send(
        &self,
        exchange: &str,
        queue_name: &str,
        routing_key: &str,
        payload: &str,
    ) -> InnerResult<()> {
        let chan = self
            .get_conn()
            .await?
            .create_channel()
            .await
            .map_err(MqerError::ExeError)?;

        let () = chan
            .exchange_declare(
                exchange,
                ExchangeKind::Topic,
                ExchangeDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .map_err(MqerError::ExeError)?;

        let queue = chan
            .queue_declare(
                queue_name,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .map_err(MqerError::ExeError)?;

        chan.queue_bind(
            queue.name().as_str(),
            exchange,
            routing_key,
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await
        .map_err(MqerError::ExeError)?;

        let payload = payload.as_bytes();

        chan.basic_publish(
            exchange,
            routing_key,
            BasicPublishOptions::default(),
            payload,
            BasicProperties::default(),
        )
        .await
        .map_err(MqerError::ExeError)?
        .await
        .map_err(MqerError::ExeError)?;
        Ok(())
    }

    pub async fn topic_receive<D: ConsumerDelegate + 'static>(
        &self,
        exchange: &str,
        queue_name: &str,
        routing_key: &str,
        tag: &str,
        delegate: D,
    ) -> InnerResult<()> {
        let chan = self
            .get_conn()
            .await?
            .create_channel()
            .await
            .map_err(MqerError::ExeError)?;

        chan.exchange_declare(
            exchange,
            ExchangeKind::Topic,
            ExchangeDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .map_err(MqerError::ExeError)?;

        let queue = chan
            .queue_declare(
                queue_name,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .map_err(MqerError::ExeError)?;

        chan.queue_bind(
            queue.name().as_str(),
            exchange,
            routing_key,
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await
        .map_err(MqerError::ExeError)?;

        let consumer = chan
            .basic_consume(
                queue.name().as_str(),
                tag,
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .map_err(MqerError::ExeError)?;

        consumer.set_delegate(delegate);

        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use deadpool_lapin::lapin::{
        message::DeliveryResult, options::BasicAckOptions,
    };

    use crate::library::{cfg, mqer::Subscriber, Mqer};

    #[tokio::test]
    async fn test_basic_send() {
        cfg::init(&"../fixtures/config.toml".to_string());
        // let mqer = init("inpay.dev.queue", Some("inpay.dev.exchange"),
        // Some("inpay.dev.routine")).await;
        let mqer = Mqer::init();

        for i in 0..10 {
            let msg = format!("#{i} Testtest");
            let confirm = mqer.basic_send("inpay.dev.queue", &msg).await;
            match confirm {
                Ok(()) => tracing::info!("[x] 消息已发送成功！{}", msg),
                Err(e) => tracing::error!("{:?}", e),
            };

            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }

    #[tokio::test]
    async fn test_basic_receive() {
        cfg::init(&"../fixtures/config.toml".to_string());
        let mqer = Mqer::init();
        let func = |message: String| {
            eprintln!("{message}");
        };
        let delegate = Subscriber::new(func);
        // tokio::spawn(async move {
        mqer.basic_receive("inpay.dev.queue", "inpay.dev.tag", delegate)
            .await
            .unwrap();
        // });
        // loop{}
    }

    #[tokio::test]
    #[ignore]
    async fn test_topic_send() {
        cfg::init(&"../fixtures/config.toml".to_string());
        let mqer = Mqer::init();
        for i in 0..10 {
            let msg = format!("#{i} Testtest");
            let confirm = mqer
                .topic_send(
                    "inpay.dev.exchange",
                    "inpay.dev.queue",
                    "inpay.dev.routine",
                    &msg,
                )
                .await;
            match confirm {
                Ok(()) => tracing::info!("[x] 消息已发送成功！{}", msg),
                Err(e) => tracing::error!("{:?}", e),
            };

            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }

    #[tokio::test]
    #[ignore]
    async fn test_topic_receive() {
        cfg::init(&"../fixtures/config.toml".to_string());
        let mqer = Mqer::init();
        mqer.topic_receive(
            "inpay.dev.queue",
            "inpay.dev.exchange",
            "inpay.dev.routine",
            "inpay.dev.tag",
            move |delivery: DeliveryResult| async move {
                tracing::debug!("aaa");
                let delivery = match delivery {
                    Ok(Some(delivery)) => delivery,
                    Ok(None) => {
                        tracing::error!("None ");
                        return;
                    }
                    Err(err) => {
                        tracing::error!(
                            "Failed to consume queue message {}",
                            err
                        );
                        return;
                    }
                };

                let message = String::from_utf8_lossy(&delivery.data);
                tracing::info!("Received a message: {}", message);

                delivery.ack(BasicAckOptions::default()).await.unwrap();
            },
        )
        .await
        .unwrap();
    }
}
