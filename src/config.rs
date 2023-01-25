use config::{builder::DefaultState, Config, ConfigBuilder, ConfigError, File, FileFormat};

pub fn build_config() {
    match build_builder() {
        Ok(config) => {
            println!("Config built");
        }
        Err(e) => {
            println!("Err initing config: {:?}", e);
        }
    }
}

fn build_builder() -> Result<ConfigBuilder<DefaultState>, ConfigError> {
    let builder = Config::builder()
        .set_default("example_key", "big_chungus")?
        .set_default("library_location", "~/Documents/Manga Library/")?
        .add_source(File::new("config/settings", FileFormat::Toml));

    Ok(builder)
}
