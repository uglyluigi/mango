pub mod categorizer_service;
pub mod config;
pub mod ui;

fn main() {
    config::build_config();
    let mut lib = categorizer_service::library::build_library("./MangaLibrary".into());
    println!("{}", lib);
}
