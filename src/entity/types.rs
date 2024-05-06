use serde::{Deserialize, Serialize};

#[derive(sqlx::Type, Debug, Clone, Copy, Serialize, Deserialize)]
#[sqlx(type_name = "currency")]
pub enum Currency {
    USD,
    EUR,
    GBP,
    CNY,
}

#[derive(sqlx::Type, Debug, Clone, Copy, Serialize, Deserialize)]
#[sqlx(type_name = "language")]
pub enum Language {
    #[sqlx(rename = "en-US")]
    EnUs,
    #[sqlx(rename = "zh-CN")]
    ZhCn,
    #[sqlx(rename = "fr-FR")]
    FrFr,
    #[sqlx(rename = "es-ES")]
    EsEs,
}
