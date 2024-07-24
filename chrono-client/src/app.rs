use std::collections::HashMap;
use futures::{StreamExt};
use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::Message;
use leptos::logging::log;
use leptos::*;
use serde::{Deserialize, Serialize};
use storage::{get_token};
use views::empty::Empty;
use views::home::Home;
use views::sign_login::SignLoginView;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageData {
    HasNewInfo(String)
}

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
    let (has_new_info_read, has_new_info_write) = create_signal::<HashMap<String, i64>>(HashMap::new());
    let ws = WebSocket::open("ws://127.0.0.1:65000/ws").unwrap();
    let (write, mut read) = ws.split();
    spawn_local(async move {
        while let Some(Ok(msg)) = read.next().await {
            match msg {
                Message::Text(_) => {}
                Message::Bytes(data) => {
                    if let Ok(data) = serde_json::from_slice::<MessageData>(&data) {
                        match data {
                            MessageData::HasNewInfo(peer_id) => {
                                let mut data_map = has_new_info_read.get();
                                data_map.insert(peer_id, 1);
                                has_new_info_write.set(data_map);
                            }
                        }
                    }
                }
            }
        }
        log!("{}", "WebSocket Closed");
    });
    let token = get_token();
    let mut current_token = "Home";
    if token.is_empty() {
        current_token = "Login";
    }
    let (router, router_set) = create_signal(current_token.to_string());
    let (body, set_body) = create_signal(view! {<Home router_set has_new_info_read/>});
    create_resource(
        move || router.get(),
        move |router| async move {
            let v = match router.as_str() {
                "Login" => view! {<SignLoginView router_set/>},
                "Home" => view! {<Home router_set has_new_info_read/>},
                _ => view! {<Empty/>},
            };
            set_body.set(v);
        },
    );

    view! {{move|| body.get()}}
}
