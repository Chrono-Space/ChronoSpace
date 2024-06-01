use futures::channel::mpsc::unbounded;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::Message;
use leptos::logging::log;
use leptos::prelude::*;
use std::sync::Arc;
use views::sign_login::SignLoginView;
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn App() -> impl IntoView {
    let ws = WebSocket::open("ws://127.0.0.1:65000/ws").unwrap();
    let (mut write, mut read) = ws.split();
    let (s, mut r) = unbounded::<Message>();
    let s = Arc::new(s);
    let ss = s.clone();
    let sss = s.clone();
    spawn_local(async move {
        while let data = r.select_next_some().await {
            log!("{:?}", data);
            write.send(data).await.unwrap();
        }
    });

    spawn_local(async move {
        while let Some(msg) = read.next().await {
            log!("{}", format!("1. {:?}", msg));
        }
        log!("{}", "WebSocket Closed");
    });
    view! {
        <main class="container">
            <SignLoginView/>
        </main>
    }
}
