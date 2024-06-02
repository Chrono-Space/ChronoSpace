use futures::channel::mpsc::channel;
use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::Message;
use leptos::ev::SubmitEvent;
use leptos::logging::log;
use leptos::Attribute::String;
use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use std::sync::Arc;
use views::empty::Empty;
use views::friend_list::FriendList;
use views::sign_login::SignLoginView;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    let ws = WebSocket::open("ws://127.0.0.1:65000/ws").unwrap();
    let (mut write, mut read) = ws.split();
    let (s, mut r) = channel::<Message>(100);
    let s = Arc::new(s);
    let ss = s.clone();
    let sss = s.clone();
    spawn_local(async move {
        write.send(Message::Text("data".to_string())).await.unwrap();
    });

    spawn_local(async move {
        while let Some(msg) = read.next().await {
            log!("{}", format!("1. {:?}", msg));
        }
        log!("{}", "WebSocket Closed");
    });
    let (router, router_set) = create_signal("Login".to_string());

    let message = move || match router.get().as_str() {
        "Login" => view! {<SignLoginView router_set/>},
        "Home" => view! {<Empty/>},
        _ => view! {<Empty/>},
    };
    view! {
        <main class="container">
            { message }
        </main>
    }
}
