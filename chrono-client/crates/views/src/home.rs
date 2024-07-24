use std::collections::HashMap;
use components::chat::chat_window::ChatWindow;
use components::community_list::CommunityList;
use components::friend_list::FriendList;
use components::group::GroupInfo;
use data_types::friend::Friends;
use leptos::*;

#[component]
pub fn Home(router_set: WriteSignal<String>, has_new_info_read: ReadSignal<HashMap<String, i64>>) -> impl IntoView {
    let (read_select_friend, write_select_friend) = create_signal::<Friends>(Friends::default());
    let (read_select_group, write_select_group) = create_signal::<GroupInfo>(GroupInfo::default());

    view! {
        <div class="home">
            <div>
                <CommunityList write_select_group/>
            </div>
            <div>
                <FriendList write_select_friend router_set has_new_info_read/>
            </div>
            <div class="home-chat-window">
                <ChatWindow read_select_friend router_set has_new_info_read/>
            </div>
        </div>
    }
}
