use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGroupRequest {
    pub name: String,
    pub remark: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBwGroupRequest {
    pub group_id: i64,
    pub name: Option<String>,
    pub remark: Option<String>,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct DeleteBwGroupRequest {
    pub group_id: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReadBwGroupRequest {
    pub group_ids: Vec<i64>,
}
