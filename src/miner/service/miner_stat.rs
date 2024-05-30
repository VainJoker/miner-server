use std::{sync::Arc, time::Duration};

use serde::{Deserialize, Serialize};
use tokio::time::interval;

use crate::{
    library::{
        cfg,
        error::{AppError, AppResult},
    },
    miner::bootstrap::AppState,
};

#[derive(Clone)]
pub struct Server {
    coin_stat: Arc<CoinStat>,
}

impl Server {
    pub fn init() -> Server {
        let cfg = cfg::config();
        let coin_stat_host = &cfg.miner.coin_stat.host;
        let coin_stat_duration =
            Duration::from_secs(cfg.miner.coin_stat.frequency);
        let coins = cfg.miner.coins.clone();
        let coin_stat =
            CoinStat::new(coin_stat_host, coins, coin_stat_duration);
        Server {
            coin_stat: Arc::new(coin_stat),
        }
    }

    pub fn serve(self, app_state: Arc<AppState>) -> AppResult<()> {
        let coin_stat = self.coin_stat;

        tokio::spawn(async move {
            let mut interval = interval(coin_stat.duration);
            let mut redis = app_state.get_redis().await.unwrap();
            loop {
                interval.tick().await;

                match coin_stat.get_data().await {
                    Ok(res) => {
                        redis
                            .set(
                                "coin_stat",
                                &serde_json::to_string(&res).unwrap(),
                            )
                            .await
                            .unwrap();
                        tracing::trace!("Successfully fetched coin stats")
                    }
                    Err(e) => {
                        tracing::error!("Error fetching coin stats: {:?}", e)
                    }
                }
            }
        });

        Ok(())
    }

    pub fn shutdown(&self) -> AppResult<()> {
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoinData {
    id: String,
    coin: String,
    name: String,
    #[serde(rename = "type")]
    type_: String,
    algorithm: String,
    network_hashrate: f64,
    difficulty: f64,
    reward: f64,
    reward_unit: String,
    reward_block: f64,
    price: f64,
    volume: f64,
    updated: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoinStat {
    pub host: String,
    pub coins: String,
    pub duration: Duration,
}

pub type CoinSymbol<'a> = &'a str;

impl CoinStat {
    pub fn new(host: &str, coins: Vec<String>, duration: Duration) -> CoinStat {
        let coins = coins.join(",");
        CoinStat {
            host: host.to_string(),
            coins,
            duration,
        }
    }

    pub async fn get_data(&self) -> AppResult<Vec<CoinData>> {
        let url = format!("{}?list={}", self.host, self.coins);
        let client = reqwest::Client::new();
        let response = client.get(url).send().await.map_err(|e| {
            let es = format!("Error occurred while getting coin stat : {}", e);
            tracing::error!(es);
            anyhow::anyhow!(es)
        })?;
        if !response.status().is_success() {
            let es = format!(
                "Error occurred while getting coin stat : {}",
                response.status()
            );
            tracing::error!(es);
            return Err(AppError::Anyhow(anyhow::anyhow!(es)));
        }
        Ok(response.json().await.map_err(|e| {
            let es = format!("Error occurred while getting coin stat : {}", e);
            tracing::error!(es);
            anyhow::anyhow!(es)
        })?)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::miner::service::miner_stat::CoinStat;

    #[tokio::test]
    async fn get_coin_stat_works() {
        let host = "https://api.minerstat.com/v2/coins";
        let coins =
            vec!["BTC".to_string(), "BCH".to_string(), "BSV".to_string()];
        let coin_stat =
            CoinStat::new(host, coins, Duration::from_secs(60 * 60));
        let list = ["BTC", "BCH", "BSV"];
        let res = coin_stat.get_data().await.unwrap();
        assert_eq!(res.len(), list.len());
    }
}
