use std::fs::File;
use ron::de::from_reader;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub title: String,
    pub size: (u32, u32),
    pub resizable: bool,
    pub fullscreen: bool,
    pub maximized: bool,
    pub grid_size: (i32, i32),
}

impl Config {
    pub fn load() -> Config {
        let input_path = format!("{}/config/config.ron", env!("CARGO_MANIFEST_DIR"));
        let f = File::open(&input_path).expect("Failed opening file");
        let config: Config = match from_reader(f) {
            Ok(x) => x,
            Err(e) => {
                println!("Failed to load config: {}", e);

                std::process::exit(1);
            }
        };

        return config;
    }
}