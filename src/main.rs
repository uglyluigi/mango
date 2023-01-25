pub mod categorizer_service;
pub mod config;
pub mod ui;

fn main() {
    let mut lib = categorizer_service::library::build_library("./MangaLibrary".into());
    config::MANGO_CONFIG.theme();
    categorizer_service::library::serialize_to_disk(lib);

    ui::show();
}
