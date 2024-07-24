use crate::group::{Group, GroupInfo};
use crate::group_member_list::GroupList;
use configs::CHRONO_IM_URL;
use leptos::*;

#[component]
pub fn CommunityList(write_select_group: WriteSignal<GroupInfo>) -> impl IntoView {
    let list = GroupList {
        list: vec![
            GroupInfo {
                avatar: "avatar/6378806623340270811.webp".to_string(),
                nickname: "123".to_string(),
            },
            GroupInfo {
                avatar: "avatar/6378806623340270811.webp".to_string(),
                nickname: "123".to_string(),
            },
        ],
    };
    let (data, write) = create_signal(list);
    view! {
        <div class="community-list">
            <For
                each=move || data.get().list.into_iter().enumerate()
                    key=|(_, state)| state.avatar.clone()
                    children=move |(index, group_info)| {
                        let group1 = group_info.clone();
                        view! {
                            <div on:click=move |_| {
                                write_select_group
                                    .update(|x| {
                                        x.avatar = group1.avatar.clone();
                                        x.nickname = group1.nickname.clone();
                                    })
                            }>
                                <Group group_info/>
                            </div>
                        }
                    }
                />
        </div>
    }
}
