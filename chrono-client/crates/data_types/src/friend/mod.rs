pub mod friend_add;
pub mod friend_list;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Friends {
    pub id: i64,
    pub pub_key: String,
    pub avatar: String,
    pub nickname: String,
    pub is_deleted: u8,
    pub updated_at: i64,
    pub created_at: i64,
}
