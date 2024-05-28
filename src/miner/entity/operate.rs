use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct OperateRequest{
    pub macs: String,
    pub action: usize,
    pub params: Value
}
