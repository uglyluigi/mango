use std::path::Path;

use crate::{
    categorizer_service::{
        self,
        library::{Library, LIBRARY},
    },
    config::MANGO_CONFIG,
};

#[tauri::command]
pub fn get_library() -> Library {
    (*categorizer_service::library::LIBRARY).clone()
}

#[tauri::command]
pub fn get_resource_server_url() -> String {
    MANGO_CONFIG.resource_server_url()
}

#[tauri::command]
pub fn get_chapter_list(series: String) -> Vec<String> {
    let mut ret = vec![];

    let series = LIBRARY.series_by_name(series).unwrap();

    for chap in &series.chapters {
        ret.push(format!("Chapter {}", chap.chapter_number));
    }

    ret
}

#[tauri::command]
pub fn get_chapter_list_2(series: String) -> Vec<(i32, String)> {
    let mut ret = vec![];

    let series = LIBRARY.series_by_name(series).unwrap();

    for chap in &series.chapters {
        ret.push((
            chap.chapter_number,
            format!("Chapter {}", chap.chapter_number),
        ));
    }

    ret
}

#[tauri::command]
pub fn get_cover(series: String) -> Vec<u8> {
    match LIBRARY.series_by_name(series) {
        Some(series) => get_img_bytes(&series.covers[0].path)
            .expect(&format!("Failed to get cover bytes of {}", series.title)),
        None => vec![],
    }
}

// Returns the bytes of each image inside of the chapter, maybe in order? I am not sure.
#[tauri::command]
pub fn get_chapter_images(series: String, chapter: i32) -> Vec<Vec<u8>> {
    let mut ret: Vec<Vec<u8>> = vec![];
    let series = LIBRARY.series_by_name(series);

    match series {
        Some(series) => match series.chapter(chapter) {
            Some(chap) => {
                for path in &chap.image_paths {
                    match get_img_bytes(path) {
                        Ok(bytes) => ret.push(bytes),
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
            }
            None => eprintln!(
                "Couldn\'t find chapter with number {} in series {}",
                chapter, series.title
            ),
        },
        None => (),
    }

    ret
}

fn get_img_bytes<P>(path: P) -> Result<Vec<u8>, std::io::Error>
where
    P: AsRef<Path>,
{
    let reader = image::io::Reader::open(path)?;

    match reader.decode() {
        Ok(dyn_img) => Ok(dyn_img.into_bytes().to_vec()),
        Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
    }
}
