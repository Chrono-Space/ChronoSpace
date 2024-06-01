use data_types::friend::friend_add::{FriendAddReq, FriendAddRes};
use data_types::friend::friend_list::{FriendListReq, FriendListRes};
use leptos::logging::log;
use leptos::prelude::*;
use wasm_http::http_ctx::HttpCtx;
use leptos_reactive::create_resource;

#[component]
pub fn FriendList() -> impl IntoView {
    let (friend_list, friend_list_set) = signal(FriendListReq {
        page_no: 1,
        page_size: 20,
    });

    let (friend_list_res, friend_list_res_set) = signal(FriendListRes::default());

    ///     <Await
    ///         future=|| fetch_monkeys(3)
    ///         let:data
    ///     >
    ///         <p>{*data} " little monkeys, jumping on the bed."</p>
    ///     </Await>

    create_resource(
        move || friend_list.get(),
        move |value| async move {
            let http_ctx = HttpCtx {
                token: String::from(
                    "+0lnk2qjYhe0M2y5qawvDgTkzslPUXuQRaAdEbPrI9NBfYesBqh34PgTwnbICnx8",
                ),
            };
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
