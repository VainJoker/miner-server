use serde::{Deserialize, Serialize};

use crate::library::error::{AppError, AppResult};

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

pub struct CoinStat<'a> {
    pub host: &'a str,
}

pub type CoinSymbol<'a> = &'a str;

impl CoinStat<'_> {
    pub fn new<'a>(host: &'a str) -> CoinStat<'a> {
        CoinStat { host }
    }

    pub fn get_coin_stat(
        &self,
        list: &Vec<CoinSymbol>,
    ) -> AppResult<Vec<CoinData>> {
        let coins = list.join(",");
        let url = format!("{}?list={}", self.host, coins);
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
        let host = "https://api.minerstat.com/v2/coins";
        let coin_stat = CoinStat::new(host);
        let list = vec!["BTC", "BCH", "BSV"];
        let res = coin_stat.get_coin_stat(&list).unwrap();
        assert_eq!(res.len(), list.len());
    }
}
