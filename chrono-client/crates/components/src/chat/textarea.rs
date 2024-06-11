use chrono::Utc;
use leptos::*;
use leptos::leptos_dom::log;
use data_types::chat_list::ChatInfo;
use data_types::chat_send::{SendChatReq, SendChatRes};
use data_types::friend::Friends;
use wasm_http::http_ctx::HttpCtx;

const ENTER_KEY: u32 = 13;

#[component]
pub fn Textarea(write_send_content: WriteSignal<Vec<ChatInfo>>, read_friend: ReadSignal<Friends>) -> impl IntoView {
    let (chat_content, write_chat_content) = create_signal(String::new());
    let (current_chat, write_current_chat) = create_signal(SendChatReq::default());
    let save = move |content:String| {
        write_chat_content.set(content);
    };
    let send = move |data| {
        write_current_chat.set(data);
    };
    create_resource(
        move || current_chat.get(),
        move |chat| async move {
            if !chat.data.is_empty() {
                if let Ok(Some(res)) = HttpCtx::default().post::<SendChatReq, SendChatRes>("/api/chat/send", &SendChatReq {
                    sender: chat.sender.to_string(),
                    receiver: chat.receiver.to_string(),
                    data_type: 1,
                    data: chat.data.to_string(),
                }).await {
                    log!("{:?}", chat);
                    write_send_content.update(move |data| {
                        data.push(res.data);
                    });
                    write_chat_content.set(String::new());
                }
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
                    let data = event_target_value(&ev).trim().to_string();
                    let req = SendChatReq{
                        sender: "".to_string(),
                        receiver: read_friend.get().peer_id.to_string(),
                        data_type: 1,
                        data};
                    send(req);
                }
            }
        >
        </textarea>
    }
}