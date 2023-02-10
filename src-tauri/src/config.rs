use std::{
    io::{self, BufWriter, Write},
    path::PathBuf,
};

use derive_getters::Getters;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref MANGO_CONFIG: Config = init_config();
}

#[derive(Serialize, Deserialize)]
pub enum Theme {
    Dark,
    Light,
}

impl Default for Theme {
    fn default() -> Self {
        Self::Light
    }
}

#[derive(Serialize, Deserialize, Getters)]
pub struct Config {
    theme: Theme,
    max_columns: usize,
    max_rows: usize,
    library_path: PathBuf,
    resource_server_port: u16,
    resource_server_domain: String,
}

impl Config {
    pub fn resource_server_url(&self) -> String {
        format!("https://{}:{}/", self.resource_server_domain, self.resource_server_port)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: Default::default(),
            max_columns: 5,
            max_rows: 100,
            library_path: "./MangaLibrary".into(),
            resource_server_port: 1420,
            resource_server_domain: String::from("localhost"),
        }
    }
}


const CONFIG_FILE_NAME: &'static str = "config.json";

pub fn write_config(config: &Config) -> io::Result<()> {
    let f = std::fs::File::create(CONFIG_FILE_NAME)?;
    let mut f = BufWriter::new(f);

    match serde_json::to_string(&config) {
        Ok(json) => {
            println!("Wrote config");
            f.write_all(json.as_bytes())
        }
        Err(e) => Err(io::Error::from(e)),
    }
}

fn init_config() -> Config {
    let config_file_path = PathBuf::from(CONFIG_FILE_NAME);

    if config_file_path.exists() {
        println!("Reading present config");
        read_config().expect("Error reading config file")
    } else {
        println!("No config file detected");
        let config = Config::default();
        match write_config(&config) {
            Ok(_) => config,
            Err(e) => panic!("Error writing config file: {:?}", e),
        }
    }
}

fn read_config() -> io::Result<Config> {
    let bytes = std::fs::read(CONFIG_FILE_NAME)?;
    let config: Config = serde_json::from_str(
        String::from_utf8(bytes)
            .expect("Failed to read library file into utf8 string")
            .as_str(),
    )
    .expect("Failed to deserialize config struct from config file");

    Ok(config)
}
