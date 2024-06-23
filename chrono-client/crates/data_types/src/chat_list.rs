use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChatListReq{
    /// 接收人
    pub receiver: String,
    pub page_no: u64,
    pub page_size: u64,
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChatListRes {
    pub list: Vec<ChatInfo>
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatInfo {
    pub id: i64,
    pub data_type: u8,
    pub data: String,
    pub is_sender: u8,
    pub status: u8,
    pub is_readed: u8,
    pub is_deleted: u8,
}
