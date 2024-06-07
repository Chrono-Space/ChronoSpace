use leptos::*;
use components::chat::chat_window::ChatWindow;
use components::friend_list::FriendList;
use data_types::friend::Friends;

#[component]
pub fn Home() -> impl IntoView {
    let (read_select_friend, write_select_friend) = create_signal::<Friends>(Friends::default());
    view! {
        <div class="home">
            <div>
                <FriendList write_select_friend/>
            </div>
            <div class="home-chat-window">
                <ChatWindow read_select_friend/>
            </div>
        </div>
    }
}
