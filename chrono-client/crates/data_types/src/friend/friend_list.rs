use crate::friend::Friends;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct FriendListReq {
    pub page_no: i64,
    pub page_size: i64,
}

#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct FriendListRes {
    pub list: Vec<Friends>,
}
