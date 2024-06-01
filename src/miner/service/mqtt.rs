use std::sync::Arc;

use rumqttc::{AsyncClient, Event, EventLoop, Incoming, MqttOptions, QoS};
use tokio::sync::Mutex;

use crate::{
    library::cfg,
    miner::{bootstrap::AppState, entity::mqtt::Message},
};

#[derive(Clone)]
pub struct Server {
    pub client: AsyncClient,
    pub event_loop: Arc<Mutex<EventLoop>>,
}

impl Server {
    pub async fn init() -> Self {
        let cfg = cfg::config();
        let mqtt_cfg = &cfg.miner.mqtt;
        let mut mqtt_opts = MqttOptions::new(
            &mqtt_cfg.client_id,
            &mqtt_cfg.host,
            mqtt_cfg.port,
        );
        mqtt_opts
            .set_keep_alive(std::time::Duration::from_secs(mqtt_cfg.keepalive));
        mqtt_opts.set_credentials(&mqtt_cfg.username, &mqtt_cfg.password);
        let (client, event_loop) = AsyncClient::new(mqtt_opts, 10);
        for topic in &mqtt_cfg.topics {
            match client.subscribe(&topic.topics, QoS::AtMostOnce).await {
                Ok(_) => {}
                Err(e) => {
                    tracing::error!(
                        "Error occurred while subscribing to topic {}: {}",
                        topic.topics,
                        e
                    );
                }
            }
        }
        Self {
            client,
            event_loop: Arc::new(Mutex::new(event_loop)),
        }
    }

    pub async fn serve(&mut self, _app_state: Arc<AppState>) {
        tracing::debug!("mqtt started");
        let ep = self.event_loop.clone();
        tokio::spawn(async move {
            loop {
                let result = {
                    let mut ep = ep.lock().await;
                    ep.poll().await
                };
                match result {
                    Ok(Event::Incoming(Incoming::Publish(p))) => {
                        tracing::trace!(
                            "Topic: {}, Payload: {:?}",
                            p.topic,
                            p.payload
                        );
                        match serde_json::from_slice::<Message>(
                            p.payload.as_ref(),
                        ) {
                            Ok(message) => {
                                eprintln!("{:#?}", message);
                                // TODO: 传入message 和 topic , 存redis
                            }
                            Err(e) => {
                                tracing::error!(
                                    "Error occurred while deserializing payload: {}",
                                    e
                                );
                            }
                        }
                    }
                    Ok(Event::Incoming(i)) => {
                        tracing::trace!("Incoming = {i:?}");
                    }
                    Ok(Event::Outgoing(o)) => {
                        tracing::trace!("Outgoing = {o:?}");
                    }
                    Err(e) => {
                        tracing::error!("Mqtt Connection Error = {e:?}");
                    }
                }
            }
        });
    }

    pub async fn shutdown(&self) {
        tracing::info!("mqtt stopped");
        match self.client.disconnect().await {
            Ok(_) => {}
            Err(e) => {
                tracing::error!("Error occurred while disconnecting: {}", e);
            }
        }
    }
}

// fn helper(message: Message, app_state: Arc<AppState>) {
//     match message {
//         Message::MessageStatus(s) => {
//             let redis = app_state.get_redis().clone();
//             s.
//         }
//         Message::MessageUpdate(u) => {}
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_mqtt_init() {
        cfg::init(&"./fixtures/config.toml".to_string());
        // let mut mqtt = Server::init().await;
        // mqtt.serve().await;
        // let shutdown_handle = tokio::spawn(async move {
        //     tokio::time::sleep(time::Duration::from_secs(5)).await;
        //     mqtt.shutdown().await;
        // });
        // tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        // let _ = shutdown_handle.await;
    }
}
