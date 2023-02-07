#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::io::Write;

use categorizer_service::library::Library;
use config::MANGO_CONFIG;
use warp::{http::Response, hyper::StatusCode, Filter};

use crate::categorizer_service::library::LIBRARY;

mod categorizer_service;
mod config;
mod commands;
use commands::*;

fn main() {
    // channel used to shut down warp server
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.spawn(async {
        println!("Starting warp server");

        let resource_get = warp::path!("covers" / String).map(|series_name: String| {
            let deco_series_name = urlencoding::decode(&series_name).unwrap();

            match LIBRARY.series_by_name(deco_series_name.to_string()) {
                Some(serie) => {
                    let mut img_data = vec![];

                    for cover in &serie.covers {
                        let path = cover.path.to_str().unwrap();
                        let b64_data = image_base64::to_base64(path);

                        // FIXME should support PNG and other formats too
                        // FIXME does this even require base64 encoding?
                        img_data.push(format!("{}", b64_data));
                    }

                    Response::builder()
                        .status(StatusCode::OK)
                        .header("Access-Control-Allow-Origin", "http://127.0.0.1:1430")
                        .body(serde_json::to_string(&img_data).unwrap())
                }
                None => Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body("".to_owned()),
            }
        });

        let (_, server) = warp::serve(resource_get).bind_with_graceful_shutdown(
            (
                [127, 0, 0, 1],
                config::MANGO_CONFIG.resource_server_port().clone(),
            ),
            async move {
                tokio::signal::ctrl_c()
                    .await
                    .expect("Failed to shutdown warp server")
            },
        );

        server.await;
        // Often the runtime exits before stdout actually gets a chance to flush,
        // so this message doesn't always appear, but does sometimes.
        // either way, it is shutting down gracefully
        println!("Shutting down warp server");
        std::io::stdout().flush().unwrap();
    });

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_library,
            get_resource_server_url
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
