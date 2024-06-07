use std::hash::{DefaultHasher, Hash};
use leptos::*;
use leptos::logging::log;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Hash, Eq, Deserialize, Serialize)]
pub struct Chat{
    pub created_at: i64,
    pub is_sender: u8,
    pub chat_info: ChatInfo,
    pub to: String,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq, Deserialize, Serialize)]
pub enum ChatInfo {
    Text(String),
    Img(String),
    File(Vec<u8>),
    None
}

impl Default for ChatInfo {
    fn default() -> Self {
        Self::None
    }
}

#[component]
pub fn ChatList(read_send_content: ReadSignal<Vec<Chat>>, ) -> impl IntoView {
    view! {
        <For
            each=move || read_send_content.get().into_iter().enumerate()
            key=|(_, state)| state.created_at
            children=move |(index, chat_info)| {
                log!("{:?}", chat_info.chat_info);
                match chat_info.chat_info {
                    ChatInfo::Text(data) => {
                        view! {
                            <div>
                                <p>{data}</p>
                            </div>
                        }
                    }
                    ChatInfo::Img(_) => view! { <div></div> },
                    ChatInfo::File(_) => view! { <div></div> },
                    _ => {view! {<div></div>}}
            }
            }
        />
    }
}