#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod categorizer_service;
mod commands;
mod config;
mod resource_server;

use commands::*;

fn main() {
    // channel used to shut down warp server
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.spawn(async {
        resource_server::init().await;
    });

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_resource_server_url,
            get_chapter_list,
            get_all_titles,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
