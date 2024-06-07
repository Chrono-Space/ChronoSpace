use data_types::friend::friend_list::{FriendListReq, FriendListRes};
use leptos::logging::log;
use leptos::*;
use ::macros::if_else;
use data_types::friend::Friends;
use wasm_http::http_ctx::HttpCtx;

const NICKNAME: &'static str = "None";
#[component]
pub fn FriendList(write_select_friend: WriteSignal<Friends>) -> impl IntoView {
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
                                        x.created_at = friend1.created_at;
                                        x.updated_at = friend1.updated_at;
                                    })
                            }>
                                <img src=format!(
                                    "http://127.0.0.1:65000/api/show/{}",
                                    friend.avatar,
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