use leptos::*;
use components::chat::chat_window::ChatWindow;
use components::community_list::CommunityList;
use components::friend_list::FriendList;
use components::group::GroupInfo;
use components::group_member_list::GroupMemberList;
use components::title_bar::TitleBar;
use data_types::friend::Friends;

#[component]
pub fn Home(router_set: WriteSignal<String>) -> impl IntoView {
    let (read_select_friend, write_select_friend) = create_signal::<Friends>(Friends::default());
    let (read_select_group, write_select_group) = create_signal::<GroupInfo>(GroupInfo::default());

    view! {
        <div class="home">
            <div>
                <CommunityList write_select_group/>
            </div>
            <div>
                <FriendList write_select_friend router_set/>
            </div>
            <div class="home-chat-window">
                <ChatWindow read_select_friend router_set/>
            </div>
        </div>
    }
}
