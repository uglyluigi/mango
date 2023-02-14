use std::io::Write;
use warp::{http::Response, hyper::StatusCode, Filter};

use crate::{categorizer_service::library::LIBRARY, config};

pub async fn init() {
    println!("Starting warp server");

    let get_cover = warp::path!("covers" / String).map(|series_name: String| {
        match LIBRARY.series_by_name(decode_series_name(series_name)) {
            Some(serie) => {
                let cover = &serie.covers[0];
                let data = std::fs::read(&cover.path).expect("Failed to get bytes");

                Response::builder()
                    .status(StatusCode::OK)
                    .header("Access-Control-Allow-Origin", "http://127.0.0.1:1430")
                    .header("Content-Type", "image/jpeg")
                    .header("Content-Length", std::mem::size_of::<u8>() * data.len())
                    .body(data)
            }
            None => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(vec![]),
        }
    });

    // chapter_image/series_title/chapter_num/image_num
    let get_chapter_images = warp::path!("chapter_image" / String / i32 / i32)
        .map(|series_name: String, chapter: i32, image: i32| {
            let chapter_image = &LIBRARY
                .series_by_name(decode_series_name(series_name))
                .expect(&format!("Couldn\'t find series"))
                .chapter(chapter)
                .unwrap()
                .image_paths[image as usize];

            let bytes = std::fs::read(chapter_image).expect("Failed to get bytes!!!");

            Response::builder()
                .header("Access-Control-Allow-Origin", "http://127.0.0.1:1430")
                .header("Content-Type", "image/jpeg")
                .header("Content-Length", std::mem::size_of::<u8>() * bytes.len())
                .body(bytes)
        })
        .or(get_cover);

    let get_num_of_chapter_images = warp::path!("image_count" / String / i32)
        .map(|series_name: String, chapter_num: i32| {
            let series = LIBRARY
                .series_by_name(decode_series_name(series_name))
                .expect("Couldn\'t find series");

            Response::builder()
                .header("Access-Control-Allow-Origin", "http://127.0.0.1:1430")
                .body(format!(
                    "{}",
                    series.chapter(chapter_num).unwrap().image_paths.len()
                ))
        })
        .or(get_chapter_images);

    let (_, server) = warp::serve(get_num_of_chapter_images)
        .tls()
        .cert_path("./tls/Mango.crt")
        .key_path("./tls/Mango.key")
        .bind_with_graceful_shutdown(
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
}


fn decode_series_name(name: String) -> String {
    urlencoding::decode(&name).unwrap().into_owned()
}