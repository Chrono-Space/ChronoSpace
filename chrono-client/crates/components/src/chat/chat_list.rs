use std::hash::{Hash};
use leptos::*;
use leptos::logging::log;
use configs::CHRONO_IM_URL;
use data_types::chat_list::ChatInfo;
use data_types::friend::Friends;

#[component]
pub fn ChatList(read_send_content: ReadSignal<Vec<ChatInfo>>, myself: ReadSignal<Friends>, friend: ReadSignal<Friends>) -> impl IntoView {
    let (myself_avatar, _) =create_signal(format!("{}/api/show/{}", CHRONO_IM_URL, myself.get().avatar));
    let (friend_avatar, _)= create_signal(format!("{}/api/show/{}", CHRONO_IM_URL, friend.get().avatar));
    view! {
        <div class="chat-record-list">
        <For
            each=move || read_send_content.get().into_iter().enumerate()
            key=|(_, state)| state.id
            children = move |(index, chat_info)| {
                log!("{:?}", chat_info.data);
                match chat_info.data_type {
                    1 => {
                        log!("{:?}", chat_info);
                        match chat_info.is_sender {
                            1=> {
                                view!{
                                    <div class="chat-record-myself">
                                        <p class="chat-record-list-p-myself">{chat_info.data}</p>
                                        <img class="avatar-img" src={move||myself_avatar.get()}/>
                                    </div>
                                }
                            }
                            _ => {
                                view! {
                                    <div class="chat-record-other">
                                        <img class="avatar-img" src={move|| friend_avatar.get()}/>
                                        <p class="chat-record-list-p-other">{chat_info.data.clone()}</p>
                                    </div>
                                }
                            }
                        }
                    }
                    2 => view! { <div></div> },
                    3 => view! { <div></div> },
                    _ => {view! {<div></div>}}
            }
            }
        />
        {
            let doc = document();
            if let Ok(Some(content)) = doc.query_selector(".chat-record") {
                content.set_scroll_top(content.scroll_height());
            }
        }
        </div>
    }
}