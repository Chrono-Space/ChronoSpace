// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::api::process::{Command, CommandEvent};

fn main() {
    let (mut rx, mut child) = Command::new_sidecar("chrono-im")
        .expect("failed to create `my-sidecar` binary command")
        .spawn()
        .expect("Failed to spawn sidecar");

    tauri::async_runtime::spawn(async move {
        // read events such as stdout
        while let Some(event) = rx.recv().await {
            if let CommandEvent::Stdout(_line) = event {}
        }
    });
    tauri::Builder::default()
        .plugin(tauri_plugin_drag::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    child.kill().unwrap();
}
