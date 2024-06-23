use leptos::{component, create_signal, document, IntoView, Show, SignalGet, SignalSet, view};
use serde::{Deserialize, Serialize};
use configs::CHRONO_IM_URL;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GroupInfo {
    pub avatar:String,
    pub nickname: String,
}

#[component]
pub fn Group(group_info: GroupInfo) -> impl IntoView {
    let (params, write_params) = create_signal("none");
    view! {
        <div class="group-info">
        <div on:mouseover=move |x| {write_params.set("show")} on:mouseout=move |x| {write_params.set("none")}>
            <img src=format!("{}/api/show/{}",CHRONO_IM_URL, group_info.avatar)/>
        </div>
        <div>
            <Show
                 when=move || { params.get() == "show" }
                 fallback=move || {
                      view! {}
                 }
                 >
                {
                    view! {<h2 class="group-show-float">{group_info.nickname.clone()}</h2>}
                }
            </Show>
        </div>
        </div>
    }
}