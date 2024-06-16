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
                Ok(_) => {
                    tracing::debug!("Subscribed to topic {}", topic.topics);
                }
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

    pub async fn serve(&mut self, app_state: Arc<AppState>) {
        tracing::debug!("MQTT service started");
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
                            "Received message on topic: {}, Payload: {:?}",
                            p.topic,
                            p.payload
                        );
                        Self::handle_message(
                            &p.topic,
                            p.payload.as_ref(),
                            app_state.clone(),
                        )
                        .await;
                    }
                    Ok(Event::Incoming(i)) => {
                        tracing::trace!("Incoming event: {:?}", i);
                    }
                    Ok(Event::Outgoing(o)) => {
                        tracing::trace!("Outgoing event: {:?}", o);
                    }
                    Err(e) => {
                        tracing::error!("MQTT connection error: {:?}", e);
                    }
                }
            }
        });
    }

    pub async fn shutdown(&self) {
        tracing::info!("MQTT service stopping");
        if let Err(e) = self.client.disconnect().await {
            tracing::error!("Error occurred while disconnecting: {}", e);
        }
    }

    async fn handle_message(
        topic: &str,
        payload: &[u8],
        app_state: Arc<AppState>,
    ) {
        match serde_json::from_slice::<Message>(payload) {
            Ok(message) => {
                let re = regex_lite::Regex::new(
                    r"([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})",
                )
                .unwrap();
                let mac = match re
                    .captures(topic)
                    .and_then(|m| m.get(0))
                    .map(|m| m.as_str())
                {
                    Some(mac) => mac,
                    None => {
                        tracing::error!("Invalid topic: {}", topic);
                        return;
                    }
                };
                tracing::debug!("MAC: {}, Message: {:#?}", mac, message);
                if let Err(e) = message.store(app_state, mac).await {
                    tracing::error!(
                        "Error occurred while handling message: {}",
                        e
                    );
                };
            }
            Err(e) => {
                tracing::error!(
                    "Error occurred while deserializing payload: {}",
                    e
                );
            }
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::miner::bootstrap::AppState;

    #[tokio::test]
    #[ignore]
    async fn test_mqtt_init() {
        cfg::init(&"./fixtures/config.toml".to_string());
        let app_state = Arc::new(AppState::init().await);
        let mut mqtt = Server::init().await;
        mqtt.serve(app_state).await;

        let shutdown_handle = tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            mqtt.shutdown().await;
        });

        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        let _ = shutdown_handle.await;
    }
}
