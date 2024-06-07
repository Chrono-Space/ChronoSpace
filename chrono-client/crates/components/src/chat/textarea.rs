use chrono::Utc;
use leptos::*;
use leptos::leptos_dom::log;
use data_types::friend::friend_list::{FriendListReq, FriendListRes};
use data_types::friend::Friends;
use wasm_http::http_ctx::HttpCtx;
use crate::chat::chat_list::{Chat, ChatInfo};

const ENTER_KEY: u32 = 13;

#[component]
pub fn Textarea(write_send_content: WriteSignal<Vec<Chat>>, read_friend: ReadSignal<Friends>) -> impl IntoView {
    let (chat_content, write_chat_content) = create_signal(String::new());
    let (current_chat, write_current_chat) = create_signal(Chat::default());
    let save = move |content:String| {
        write_chat_content.set(content);
    };
    let send = move |content| {
        let chat = Chat{ created_at: Utc::now().timestamp_nanos_opt().unwrap_or_default(), is_sender: 1, chat_info: ChatInfo::Text(content), to: read_friend.get().peer_id };
        write_current_chat.set(chat.clone());
        write_chat_content.set(String::new());
    };
    create_resource(
        move || current_chat.get(),
        move |chat| async move {
            // let http_ctx = HttpCtx::default();
            // if let Ok(Some(data)) = http_ctx
            //     .post::<FriendListReq, FriendListRes>("/api/friend/list", &value)
            //     .await
            // {
            //     log!("{:?}", data);
            //     friend_list_res_set.set(data);
            // }
            if chat.chat_info != ChatInfo::None {
                log!("{:?}", chat);
                write_send_content.update(move |data| {
                    data.push(chat);
                });
            }
        },
    );
    view! {
        <textarea
            prop:value=chat_content
            on:focusout=move |ev: web_sys::FocusEvent| save(event_target_value(&ev))
            on:keyup=move |ev: web_sys::KeyboardEvent| {
                let ctrl_key = ev.ctrl_key();
                let key_code = ev.key_code();
                if ctrl_key && key_code == ENTER_KEY {
                    save(event_target_value(&ev));
                } else if key_code == ENTER_KEY {
                    send(event_target_value(&ev).trim().to_string());
                }
            }
        >
        </textarea>
    }
}