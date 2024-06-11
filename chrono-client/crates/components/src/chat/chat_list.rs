use std::hash::{Hash};
use std::rc::Rc;
use std::sync::Arc;
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
            key=|(_, state)| state.created_at
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
        </div>
    }
}

// #[component]
// pub fn ChatShow(chat_info: ChatInfo) -> impl IntoView {
//     view! {
//         <Show
//             when= chat_info.is_sender == 1
//             fallback=|| view! {
//                 <div class="chat-record-other">
//                     <img class="avatar-img" src={friend_avatar.get()}/>
//                     <p class="chat-record-list-p-other">{chat_info.data.clone()}</p>
//             </div> }
//         >
//             view!{
//                 <div class="chat-record-myself">
//                     <p class="chat-record-list-p-myself">{chat_info.data}</p>
//                     <img class="avatar-img" src={myself_avatar.get()}/>
//                 </div>
//             }
//         </Show>
//     }
// }