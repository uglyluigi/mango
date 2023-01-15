pub mod ui;
pub mod config;
pub mod categorizer_service;

fn main() {
    config::build_config();
    categorizer_service::library::build_library();
}
