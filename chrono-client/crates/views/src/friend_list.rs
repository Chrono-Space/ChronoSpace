use data_types::friend::friend_list::{FriendListReq, FriendListRes};
use leptos::logging::log;
use leptos::*;
use wasm_http::http_ctx::HttpCtx;
#[component]
pub fn FriendList() -> impl IntoView {
    let (friend_list, friend_list_set) = create_signal(FriendListReq {
        page_no: 1,
        page_size: 20,
    });

    let (friend_list_res, friend_list_res_set) = create_signal(FriendListRes::default());
    create_resource(
        move || friend_list.get(),
        move |value| async move {
            let http_ctx = HttpCtx::default();
            if let Ok(Some(data)) = http_ctx
                .post::<FriendListReq, FriendListRes>("/api/friend/list", &value)
                .await
            {
                log!("{:?}", data);
                friend_list_res_set.set(data);
            }
        },
    );

    view! {
        <ul>
        <For each=move || friend_list_res.get().list
                key = |friend| friend.id
                children=move |friend| {
                    view! {<li>
                        <span>{friend.id}</span>
                        </li>}
                    }
            />
        </ul>
    }
}
