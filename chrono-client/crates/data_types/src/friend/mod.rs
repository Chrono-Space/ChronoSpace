pub mod friend_add;
pub mod friend_list;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct Friends {
    pub id: i64,
    pub pub_key: String,
    pub peer_id: String,
    pub avatar: String,
    pub nickname: String,
    pub is_group: u8,
    pub is_deleted: u8,
}
