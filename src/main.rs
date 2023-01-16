pub mod ui;
pub mod config;
pub mod categorizer_service;

fn main() {
    config::build_config();
    let mut lib = categorizer_service::library::build_library();
    match categorizer_service::library::serialize_to_disk(lib) {
        Ok(()) => println!("Wrote to disk"),
        Err(e) => println!("Unable to write to disk: {:?}", e),
    }

    let library = categorizer_service::library::deserialize_from_disk().unwrap();
    println!("{}", library);
}
