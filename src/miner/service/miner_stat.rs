use std::time::Duration;
use serde::{Deserialize, Serialize};
use tokio::time::interval;
use crate::library::cfg;

use crate::library::error::{AppError, AppResult};

pub struct Server<'a>{
    coin_stat: CoinStat<'a>
}

impl Server<'_> {
    pub fn init() -> Server<'static> {
        let cfg = cfg::config();
        let coin_stat_host = &cfg.miner.coin_stat.host;
        let coins = cfg.miner.coins.clone();
        let coin_stat = CoinStat::new(coin_stat_host, coins);
        Server{
            coin_stat
        }
    }

    pub async fn serve(&self) -> AppResult<()> {
        let mut interval = interval(Duration::from_secs(60 * 60));

        loop {
            interval.tick().await;

            match self.coin_stat.get_data() {
                Ok(_) => tracing::trace!("Successfully fetched coin stats"),
                Err(e) => tracing::error!("Error fetching coin stats: {:?}", e),
            }
        }
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
pub struct CoinStat<'a> {
    pub host: &'a str,
    pub coins: String,
}

pub type CoinSymbol<'a> = &'a str;

impl CoinStat<'_> {
    pub fn new<'a>(host: &'a str, coins: Vec<String>) -> CoinStat<'a> {
        let coins = coins.join(",");
        CoinStat { host, coins }
    }

    pub fn get_data(
        &self,
    ) -> AppResult<Vec<CoinData>> {
        let url = format!("{}?list={}", self.host, self.coins);
        let client = reqwest::blocking::Client::new();
        let response = client.get(&url).send().map_err(|e| {
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
        Ok(response.json().map_err(|e| {
            let es = format!("Error occurred while getting coin stat : {}", e);
            tracing::error!(es);
            anyhow::anyhow!(es)
        })?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_coin_stat_works() {
        // let host = "https://api.minerstat.com/v2/coins";
        // let coins = vec!["BTC,BCH,BSV"];
        // let coin_stat = CoinStat::new(host,coins);
        // let list = vec!["BTC", "BCH", "BSV"];
        // let res = coin_stat.get_data().unwrap();
        // assert_eq!(res.len(), list.len());
    }
}
