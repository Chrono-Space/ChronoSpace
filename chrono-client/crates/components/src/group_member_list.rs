use crate::group::GroupInfo;
use configs::CHRONO_IM_URL;
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupList {
    pub list: Vec<GroupInfo>,
}

#[component]
pub fn GroupMemberList(write_select_friend: WriteSignal<GroupInfo>) -> impl IntoView {
    view! {}
}
