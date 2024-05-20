use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Configs {
    pub database: Database,
}

impl Configs {
    pub fn init() -> Self {
        let mut file = match std::fs::File::open("config.toml") {
            Ok(file) => file,
            Err(_) => std::fs::File::open("config.example.toml").unwrap(),
        };
    }
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub database_url: String,
}

pub static CFG: Lazy<Configs> = Lazy::new(self::Configs::init);
