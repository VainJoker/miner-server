use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Limit {
    pub page: u32,
    pub page_size: u32,
    pub offset: u32,
    pub limit: u32,
}
