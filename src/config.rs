use envy::Error;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: usize,
    pub database_url: String,
    pub local_storage_path: String,
}

impl Config {
    pub fn new() -> Result<Self, Error> {
        envy::from_env()
    }
}
