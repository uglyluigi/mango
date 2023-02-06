#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use categorizer_service::library::Library;

mod config;
mod categorizer_service;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_library() -> Library {
    (*categorizer_service::library::LIBRARY).clone()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_library])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
