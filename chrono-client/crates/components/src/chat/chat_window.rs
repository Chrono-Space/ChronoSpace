use leptos::*;
use leptos::leptos_dom::log;
use data_types::friend::friend_list::{FriendListReq, FriendListRes};
use data_types::friend::Friends;
use wasm_http::http_ctx::HttpCtx;
use crate::empty::Empty;
use super::textarea::Textarea;
use super::chat_list::{Chat, ChatList};
#[component]
pub fn ChatWindow(read_select_friend: ReadSignal<Friends>) -> impl IntoView {
    let (read_send_content, write_send_content) = create_signal::<Vec<Chat>>(vec![]);
    let (chat_window, write_chat_window) = create_signal(view! { <Empty/> });
    let (read_friend, write_friend) = create_signal(Friends::default());
    create_resource(
        move || read_select_friend.get(),
        move |friend| async move {
            write_friend.set(friend.clone());
            write_send_content.set(vec![]);
            let data = view! {
                <div class="chat-window">
                    <div class="chat-window-title">
                        <h3>{friend.nickname}</h3>
                    </div>
                    <div class="chat-record">
                        <ChatList read_send_content/>
                    </div>
                    <div class="chat-tool"></div>
                    <div class="chat-input">
                        <Textarea write_send_content read_friend/>
                    </div>
                </div>
            }.into_view();
            write_chat_window.set(data);
        },
    );
    view! { {move || chat_window.get()} }
}