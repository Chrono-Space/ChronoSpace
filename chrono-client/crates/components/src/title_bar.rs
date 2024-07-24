use leptos::*;

#[component]
pub fn TitleBar() -> impl IntoView {
    view! {
    <div data-tauri-drag-region class="titlebar">
      <div class="titlebar-button" data-tauri-event="minimize">
        <img src="https://api.iconify.design/mdi:window-minimize.svg" alt="minimize" />
      </div>
      <div class="titlebar-button" data-tauri-event="maximize">
        <img src="https://api.iconify.design/mdi:window-maximize.svg" alt="maximize" />
      </div>
      <div class="titlebar-button" data-tauri-event="close">
        <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
      </div>
    </div>
        }
}
