use std::time::Duration;

use serde_derive::{Deserialize, Serialize};
use tokio::time::interval;

use crate::library::{
    cfg,
    error::{AppError, AppResult},
};

pub struct Server<'a> {
    exchange_rate: ExchangeRate<'a>,
}

impl Server<'_> {
    pub fn init() -> Server<'static> {
        let cfg = cfg::config();
        let exchange_rate_host = &cfg.miner.exchange_rate.host;
        let exchange_rate_key = &cfg.miner.exchange_rate.key;
        let exchange_rate =
            ExchangeRate::new(exchange_rate_host, exchange_rate_key);
        Server { exchange_rate }
    }

    pub async fn serve(&self) -> AppResult<()> {
        let mut interval = interval(Duration::from_secs(60 * 60));

        loop {
            interval.tick().await;

            match self.exchange_rate.get_rate().await {
                Ok(_) => tracing::trace!("Successfully fetched exchange rate"),
                Err(e) => {
                    tracing::error!("Error fetching exchange rate: {:?}", e)
                }
            }
        }
    }

    pub fn shutdown(&self) -> AppResult<()> {
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExchangeRate<'a> {
    pub host: &'a str,
    pub key: &'a str,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponseBody {
    pub result: String,
    pub documentation: String,
    #[serde(rename = "terms_of_use")]
    pub terms_of_use: String,
    #[serde(rename = "time_last_update_unix")]
    pub time_last_update_unix: i64,
    #[serde(rename = "time_last_update_utc")]
    pub time_last_update_utc: String,
    #[serde(rename = "time_next_update_unix")]
    pub time_next_update_unix: i64,
    #[serde(rename = "time_next_update_utc")]
    pub time_next_update_utc: String,
    #[serde(rename = "base_code")]
    pub base_code: String,
    #[serde(rename = "conversion_rates")]
    pub conversion_rates: ConversionRates,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversionRates {
    #[serde(rename = "USD")]
    pub usd: i64,
    #[serde(rename = "AED")]
    pub aed: f64,
    #[serde(rename = "AFN")]
    pub afn: f64,
    #[serde(rename = "ALL")]
    pub all: f64,
    #[serde(rename = "AMD")]
    pub amd: f64,
    #[serde(rename = "ANG")]
    pub ang: f64,
    #[serde(rename = "AOA")]
    pub aoa: f64,
    #[serde(rename = "ARS")]
    pub ars: f64,
    #[serde(rename = "AUD")]
    pub aud: f64,
    #[serde(rename = "AWG")]
    pub awg: f64,
    #[serde(rename = "AZN")]
    pub azn: f64,
    #[serde(rename = "BAM")]
    pub bam: f64,
    #[serde(rename = "BBD")]
    pub bbd: f64,
    #[serde(rename = "BDT")]
    pub bdt: f64,
    #[serde(rename = "BGN")]
    pub bgn: f64,
    #[serde(rename = "BHD")]
    pub bhd: f64,
    #[serde(rename = "BIF")]
    pub bif: f64,
    #[serde(rename = "BMD")]
    pub bmd: f64,
    #[serde(rename = "BND")]
    pub bnd: f64,
    #[serde(rename = "BOB")]
    pub bob: f64,
    #[serde(rename = "BRL")]
    pub brl: f64,
    #[serde(rename = "BSD")]
    pub bsd: f64,
    #[serde(rename = "BTN")]
    pub btn: f64,
    #[serde(rename = "BWP")]
    pub bwp: f64,
    #[serde(rename = "BYN")]
    pub byn: f64,
    #[serde(rename = "BZD")]
    pub bzd: f64,
    #[serde(rename = "CAD")]
    pub cad: f64,
    #[serde(rename = "CDF")]
    pub cdf: f64,
    #[serde(rename = "CHF")]
    pub chf: f64,
    #[serde(rename = "CLP")]
    pub clp: f64,
    #[serde(rename = "CNY")]
    pub cny: f64,
    #[serde(rename = "COP")]
    pub cop: f64,
    #[serde(rename = "CRC")]
    pub crc: f64,
    #[serde(rename = "CUP")]
    pub cup: f64,
    #[serde(rename = "CVE")]
    pub cve: f64,
    #[serde(rename = "CZK")]
    pub czk: f64,
    #[serde(rename = "DJF")]
    pub djf: f64,
    #[serde(rename = "DKK")]
    pub dkk: f64,
    #[serde(rename = "DOP")]
    pub dop: f64,
    #[serde(rename = "DZD")]
    pub dzd: f64,
    #[serde(rename = "EGP")]
    pub egp: f64,
    #[serde(rename = "ERN")]
    pub ern: f64,
    #[serde(rename = "ETB")]
    pub etb: f64,
    #[serde(rename = "EUR")]
    pub eur: f64,
    #[serde(rename = "FJD")]
    pub fjd: f64,
    #[serde(rename = "FKP")]
    pub fkp: f64,
    #[serde(rename = "FOK")]
    pub fok: f64,
    #[serde(rename = "GBP")]
    pub gbp: f64,
    #[serde(rename = "GEL")]
    pub gel: f64,
    #[serde(rename = "GGP")]
    pub ggp: f64,
    #[serde(rename = "GHS")]
    pub ghs: f64,
    #[serde(rename = "GIP")]
    pub gip: f64,
    #[serde(rename = "GMD")]
    pub gmd: f64,
    #[serde(rename = "GNF")]
    pub gnf: f64,
    #[serde(rename = "GTQ")]
    pub gtq: f64,
    #[serde(rename = "GYD")]
    pub gyd: f64,
    #[serde(rename = "HKD")]
    pub hkd: f64,
    #[serde(rename = "HNL")]
    pub hnl: f64,
    #[serde(rename = "HRK")]
    pub hrk: f64,
    #[serde(rename = "HTG")]
    pub htg: f64,
    #[serde(rename = "HUF")]
    pub huf: f64,
    #[serde(rename = "IDR")]
    pub idr: f64,
    #[serde(rename = "ILS")]
    pub ils: f64,
    #[serde(rename = "IMP")]
    pub imp: f64,
    #[serde(rename = "INR")]
    pub inr: f64,
    #[serde(rename = "IQD")]
    pub iqd: f64,
    #[serde(rename = "IRR")]
    pub irr: f64,
    #[serde(rename = "ISK")]
    pub isk: f64,
    #[serde(rename = "JEP")]
    pub jep: f64,
    #[serde(rename = "JMD")]
    pub jmd: f64,
    #[serde(rename = "JOD")]
    pub jod: f64,
    #[serde(rename = "JPY")]
    pub jpy: f64,
    #[serde(rename = "KES")]
    pub kes: f64,
    #[serde(rename = "KGS")]
    pub kgs: f64,
    #[serde(rename = "KHR")]
    pub khr: f64,
    #[serde(rename = "KID")]
    pub kid: f64,
    #[serde(rename = "KMF")]
    pub kmf: f64,
    #[serde(rename = "KRW")]
    pub krw: f64,
    #[serde(rename = "KWD")]
    pub kwd: f64,
    #[serde(rename = "KYD")]
    pub kyd: f64,
    #[serde(rename = "KZT")]
    pub kzt: f64,
    #[serde(rename = "LAK")]
    pub lak: f64,
    #[serde(rename = "LBP")]
    pub lbp: f64,
    #[serde(rename = "LKR")]
    pub lkr: f64,
    #[serde(rename = "LRD")]
    pub lrd: f64,
    #[serde(rename = "LSL")]
    pub lsl: f64,
    #[serde(rename = "LYD")]
    pub lyd: f64,
    #[serde(rename = "MAD")]
    pub mad: f64,
    #[serde(rename = "MDL")]
    pub mdl: f64,
    #[serde(rename = "MGA")]
    pub mga: f64,
    #[serde(rename = "MKD")]
    pub mkd: f64,
    #[serde(rename = "MMK")]
    pub mmk: f64,
    #[serde(rename = "MNT")]
    pub mnt: f64,
    #[serde(rename = "MOP")]
    pub mop: f64,
    #[serde(rename = "MRU")]
    pub mru: f64,
    #[serde(rename = "MUR")]
    pub mur: f64,
    #[serde(rename = "MVR")]
    pub mvr: f64,
    #[serde(rename = "MWK")]
    pub mwk: f64,
    #[serde(rename = "MXN")]
    pub mxn: f64,
    #[serde(rename = "MYR")]
    pub myr: f64,
    #[serde(rename = "MZN")]
    pub mzn: f64,
    #[serde(rename = "NAD")]
    pub nad: f64,
    #[serde(rename = "NGN")]
    pub ngn: f64,
    #[serde(rename = "NIO")]
    pub nio: f64,
    #[serde(rename = "NOK")]
    pub nok: f64,
    #[serde(rename = "NPR")]
    pub npr: f64,
    #[serde(rename = "NZD")]
    pub nzd: f64,
    #[serde(rename = "OMR")]
    pub omr: f64,
    #[serde(rename = "PAB")]
    pub pab: f64,
    #[serde(rename = "PEN")]
    pub pen: f64,
    #[serde(rename = "PGK")]
    pub pgk: f64,
    #[serde(rename = "PHP")]
    pub php: f64,
    #[serde(rename = "PKR")]
    pub pkr: f64,
    #[serde(rename = "PLN")]
    pub pln: f64,
    #[serde(rename = "PYG")]
    pub pyg: f64,
    #[serde(rename = "QAR")]
    pub qar: f64,
    #[serde(rename = "RON")]
    pub ron: f64,
    #[serde(rename = "RSD")]
    pub rsd: f64,
    #[serde(rename = "RUB")]
    pub rub: f64,
    #[serde(rename = "RWF")]
    pub rwf: f64,
    #[serde(rename = "SAR")]
    pub sar: f64,
    #[serde(rename = "SBD")]
    pub sbd: f64,
    #[serde(rename = "SCR")]
    pub scr: f64,
    #[serde(rename = "SDG")]
    pub sdg: f64,
    #[serde(rename = "SEK")]
    pub sek: f64,
    #[serde(rename = "SGD")]
    pub sgd: f64,
    #[serde(rename = "SHP")]
    pub shp: f64,
    #[serde(rename = "SLE")]
    pub sle: f64,
    #[serde(rename = "SLL")]
    pub sll: f64,
    #[serde(rename = "SOS")]
    pub sos: f64,
    #[serde(rename = "SRD")]
    pub srd: f64,
    #[serde(rename = "SSP")]
    pub ssp: f64,
    #[serde(rename = "STN")]
    pub stn: f64,
    #[serde(rename = "SYP")]
    pub syp: f64,
    #[serde(rename = "SZL")]
    pub szl: f64,
    #[serde(rename = "THB")]
    pub thb: f64,
    #[serde(rename = "TJS")]
    pub tjs: f64,
    #[serde(rename = "TMT")]
    pub tmt: f64,
    #[serde(rename = "TND")]
    pub tnd: f64,
    #[serde(rename = "TOP")]
    pub top: f64,
    #[serde(rename = "TRY")]
    pub try_field: f64,
    #[serde(rename = "TTD")]
    pub ttd: f64,
    #[serde(rename = "TVD")]
    pub tvd: f64,
    #[serde(rename = "TWD")]
    pub twd: f64,
    #[serde(rename = "TZS")]
    pub tzs: f64,
    #[serde(rename = "UAH")]
    pub uah: f64,
    #[serde(rename = "UGX")]
    pub ugx: f64,
    #[serde(rename = "UYU")]
    pub uyu: f64,
    #[serde(rename = "UZS")]
    pub uzs: f64,
    #[serde(rename = "VES")]
    pub ves: f64,
    #[serde(rename = "VND")]
    pub vnd: f64,
    #[serde(rename = "VUV")]
    pub vuv: f64,
    #[serde(rename = "WST")]
    pub wst: f64,
    #[serde(rename = "XAF")]
    pub xaf: f64,
    #[serde(rename = "XCD")]
    pub xcd: f64,
    #[serde(rename = "XDR")]
    pub xdr: f64,
    #[serde(rename = "XOF")]
    pub xof: f64,
    #[serde(rename = "XPF")]
    pub xpf: f64,
    #[serde(rename = "YER")]
    pub yer: f64,
    #[serde(rename = "ZAR")]
    pub zar: f64,
    #[serde(rename = "ZMW")]
    pub zmw: f64,
    #[serde(rename = "ZWL")]
    pub zwl: f64,
}

impl ExchangeRate<'_> {
    pub fn new<'a>(host: &'a str, key: &'a str) -> ExchangeRate<'a> {
        ExchangeRate { host, key }
    }

    pub async fn get_rate(&self) -> AppResult<ConversionRates> {
        let url = format!("{}/{}/{}", self.host, self.key, "latest/USD");
        let client = reqwest::Client::new();
        let response = client.get(url).send().await.map_err(|e| {
            let es =
                format!("Error occurred while getting exchange rate : {}", e);
            tracing::error!(es);
            anyhow::anyhow!(es)
        })?;

        if !response.status().is_success() {
            let es = format!(
                "Error occurred while getting exchange rate : {}",
                response.status()
            );
            tracing::error!(es);
            return Err(AppError::Anyhow(anyhow::anyhow!(es)));
        }
        let body: ApiResponseBody = response.json().await.map_err(|e| {
            let es =
                format!("Error occurred while getting exchange rate : {}", e);
            tracing::error!(es);
            anyhow::anyhow!(es)
        })?;
        Ok(body.conversion_rates)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_rate_works() {
        let host = "https://v6.exchangerate-api.com/v6";
        let key = "83b2f3250fcbb02d93d4e3bf";
        let _rate = ExchangeRate::new(host, key).get_rate().await.unwrap();
        // eprintln!("{:#?}",rate);
    }
}
