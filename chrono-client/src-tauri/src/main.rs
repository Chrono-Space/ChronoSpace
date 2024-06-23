// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::api::process::{Command, CommandEvent};
use tauri::Manager;

fn main() {
    let (mut rx, mut child) = Command::new_sidecar("chrono-im")
        .expect("failed to create `my-sidecar` binary command")
        .spawn()
        .expect("Failed to spawn sidecar");

    tauri::async_runtime::spawn(async move {
        // read events such as stdout
        while let Some(event) = rx.recv().await {
            if let CommandEvent::Stdout(line) = event {
                // log!(line);
                // window
                //     .emit("message", Some(format!("'{}'", line)))
                //     .expect("failed to emit event");
                // write to stdin
                // child.write("message from Rust\n".as_bytes()).unwrap();
            }
        }
    });
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
