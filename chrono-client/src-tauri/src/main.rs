// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::fs::File;
use std::io::Write;
use tauri::api::path::BaseDirectory;
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
                // child.write(line.as_bytes()).unwrap();
            }
        }
    });
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            let app_data_dir = tauri::api::path::resolve_path(
                &app_handle.config(),
                app_handle.package_info(),
                &app_handle.env(),
                "",
                Some(BaseDirectory::AppData),
            ).unwrap();
            fs::create_dir_all(app_data_dir.clone()).unwrap();

            // let op = fs::OpenOptions::new().create(true).write(true).read(true).append(true).open("/avatar/").unwrap();
            let file_path = app_data_dir.join("example.txt");
            println!("{:?}", file_path);
            let mut file = File::create(file_path).expect("Unable to create file");
            file.write_all(b"Hello, Tauri!").expect("Unable to write data");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    child.kill().unwrap();
}
