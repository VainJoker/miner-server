use crate::models::{machine::Coin, types::EnergyMode};

pub mod miner_sign;

pub fn get_energy_modes(power_modes: Vec<String>) -> Vec<EnergyMode> {
    power_modes
        .into_iter()
        .filter_map(|p| match p.to_lowercase().as_str() {
            "hashrate" => Some(EnergyMode::Power),
            "lowerpower" => Some(EnergyMode::Economize),
            "ballance" => Some(EnergyMode::Balance),
            _ => None,
        })
        .collect()
}

pub fn get_coins(coin_list: Vec<String>) -> Vec<Coin> {
    coin_list
        .into_iter()
        .filter_map(|c| match c.as_str() {
            "blake2b(SC)" => Some(Coin {
                algorithm: "blake2b".to_string(),
                symbol: "SC".to_string(),
            }),
            "eaglesong(CKB)" => Some(Coin {
                algorithm: "eaglesong".to_string(),
                symbol: "CKB".to_string(),
            }),
            "blake3(ALPH)" => Some(Coin {
                algorithm: "blake3".to_string(),
                symbol: "ALPH".to_string(),
            }),
            "blake2s(KDA)" => Some(Coin {
                algorithm: "blake2s".to_string(),
                symbol: "KDA".to_string(),
            }),
            "scrypt(LTC)" => Some(Coin {
                algorithm: "scrypt".to_string(),
                symbol: "LTC".to_string(),
            }),
            "cnr(STC)" => Some(Coin {
                algorithm: "cnr".to_string(),
                symbol: "STC".to_string(),
            }),
            "lbry(LBC)" => Some(Coin {
                algorithm: "lbry".to_string(),
                symbol: "LBC".to_string(),
            }),
            "blake2bsha3(HNS)" => Some(Coin {
                algorithm: "blake2bsha3".to_string(),
                symbol: "HNS".to_string(),
            }),
            "kHeavyHash(kaspa)" => Some(Coin {
                algorithm: "kHeavyHash".to_string(),
                symbol: "KAS".to_string(),
            }),
            "kHeavyHash(KAS)" => Some(Coin {
                algorithm: "kHeavyHash".to_string(),
                symbol: "KAS".to_string(),
            }),
            &_ => None,
        })
        .collect()
}
