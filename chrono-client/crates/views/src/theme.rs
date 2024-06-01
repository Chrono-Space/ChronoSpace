use leptos::prelude::*;
use gloo_storage::Storage;
use macros::if_else;
use leptos_reactive::create_resource;
#[component]
pub fn SelectTheme() -> impl IntoView {
    let theme: String = gloo_storage::LocalStorage::get("theme").unwrap_or(String::from("dark"));
    let (read_theme, set_theme) = create_signal(theme.to_owned());
    create_resource(
        move || read_theme.get(),
        move |current_theme| async move {
            let mut doc = document().document_element().unwrap();
            let _ = doc.set_attribute("data-theme", &current_theme);
            let _ = gloo_storage::LocalStorage::set("theme", current_theme);
        }
    );
    view! {
        <button on:click=move |_| {set_theme.set(if_else!(read_theme.get() == "light", String::from("dark"), String::from("light")))}>{read_theme}</button>
    }
}