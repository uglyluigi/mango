#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use categorizer_service::library::Library;
use tokio::sync::oneshot::{self, Receiver, Sender};
use warp::Filter;

mod categorizer_service;
mod config;
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
    // channel used to shut down warp server
    let (tx, rx): (Sender<()>, Receiver<()>) = oneshot::channel();
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.spawn(async {
        println!("Starting warp server");
        
        let resource_get =
            warp::path!("covers" / String).map(|thing| format!("cover for series: {}", thing));

        let (_, server) = warp::serve(resource_get).bind_with_graceful_shutdown(([127, 0, 0, 1], 1420), async move {
            tokio::signal::ctrl_c()
                .await
                .expect("Failed to shutdown warp server")
        });

        server.await;
        println!("Shutting down warp server");
    });

    tx.send(()).unwrap();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_library])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
