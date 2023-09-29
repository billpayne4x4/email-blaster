use std::io;
use serde::Deserialize;
#[derive(Deserialize, Clone)]
pub struct Config {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub username: String,
    pub password: String,
    pub delay: u16,
    pub from: String,
}

impl Config {
    pub fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_file = std::fs::File::open(path)?;
        let config: Config = serde_json::from_reader(config_file).
            map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        Ok(config)
    }
}