use std::collections::HashMap;
use configs::CHRONO_IM_URL;
use data_types::friend::friend_list::{FriendListReq, FriendListRes};
use data_types::friend::Friends;
use leptos::logging::log;
use leptos::*;
use wasm_http::http_ctx::HttpCtx;

const NICKNAME: &'static str = "None";
#[component]
pub fn FriendList(
    write_select_friend: WriteSignal<Friends>,
    router_set: WriteSignal<String>,
    has_new_info_read: ReadSignal<HashMap<String, i64>>
) -> impl IntoView {
    let (friend_list, friend_list_set) = create_signal(FriendListReq {
        page_no: 1,
        page_size: 20,
    });

    let (friend_list_res, friend_list_res_set) = create_signal(FriendListRes::default());
    create_resource(
        move || friend_list.get(),
        move |value| async move {
            let http_ctx = HttpCtx::default();
            match http_ctx
                .post::<FriendListReq, FriendListRes>("/api/friend/list", &value)
                .await
            {
                Ok(Some(data)) => {
                    log!("{:?}", data);
                    friend_list_res_set.set(data);
                }
                Ok(None) => {
                    router_set.set("Login".to_string());
                }
                Err(e) => {
                    router_set.set("Login".to_string());
                }
            }
        },
    );
    let data = move || friend_list_res.get().list;
    view! {
        <div class="friends-list">
            <ul>
                <For
                    each=move || data().into_iter().enumerate()
                    key=|(_, state)| state.id.clone()
                    children=move |(index, friend)| {
                        let friend1 = friend.clone();
                        view! {
                            <div on:click=move |_| {
                                write_select_friend
                                    .update(|x| {
                                        x.avatar = friend1.avatar.clone();
                                        x.id = friend1.id;
                                        x.peer_id = friend1.peer_id.clone();
                                        x.nickname = friend1.nickname.clone();
                                        x.pub_key = friend1.pub_key.clone();
                                    })
                            }>
                                <img src=format!(
                                    "{}/api/show/{}",
                                    CHRONO_IM_URL, friend.avatar,
                                )/>
                                <p>{friend.nickname.to_string()}</p>
                            </div>
                        }
                    }
                />

            </ul>
        </div>
    }
}
