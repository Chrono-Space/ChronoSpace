use components::sign_login::SignLogin;
use leptos::*;
#[component]
pub fn SignLoginView(router_set: WriteSignal<String>) -> impl IntoView {
    view! {
        <SignLogin router_set/>
    }
}
