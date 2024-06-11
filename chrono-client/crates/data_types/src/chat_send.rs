use serde::{Deserialize, Serialize};
use crate::chat_list::ChatInfo;

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct SendChatReq {
    /// 发送人
    pub sender: String,
    /// 接收人
    pub receiver: String,
    /// 数据类型 1: Text 2: Img 3: File
    pub data_type: u8,
    /// 数据内容
    pub data:String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SendChatRes {
    pub data: ChatInfo
}