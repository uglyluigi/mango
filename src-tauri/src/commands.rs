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
pub fn get_chapter_list(series: String) -> Vec<(i32, String)> {
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
pub fn get_all_titles() -> Vec<String> {
    LIBRARY.series.iter().map(|x| x.title.clone()).collect()
}