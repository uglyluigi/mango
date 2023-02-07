use crate::{categorizer_service::{library::Library, self}, config::MANGO_CONFIG};

#[tauri::command]
pub fn get_library() -> Library {
    (*categorizer_service::library::LIBRARY).clone()
}

#[tauri::command]
pub fn get_resource_server_url() -> String {
    MANGO_CONFIG.resource_server_url()
}