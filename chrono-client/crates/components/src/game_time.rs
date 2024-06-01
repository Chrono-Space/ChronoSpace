use leptos::prelude::*;
use std::ops::{Div, Rem};
use std::time::Duration;

// #[component]
// pub fn GameTime(timestamp: i64) -> impl IntoView {
//     set_interval(move || { }, Duration::from_secs(1))
// }

pub fn timestamp_current(timestamp: Option<i64>) -> Option<String> {
    if timestamp.is_none() {
        return None;
    }
    let timestamp =
        time::OffsetDateTime::now_utc().unix_timestamp() - timestamp.unwrap_or_default().div(1000);
    let date = time::OffsetDateTime::from_unix_timestamp(timestamp).unwrap();
    let hour = date.hour();
    let m = date.minute();
    let s = date.second();
    if hour > 0 {
        Some(format!("{:02}:{:02}:{:02}", hour, m.rem(60), s.rem(60)))
    } else {
        Some(format!("00:{:02}:{:02}", m.rem(60), s.rem(60)))
    }
}
