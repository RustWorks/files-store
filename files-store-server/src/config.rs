use envy::Error;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub secret_key: String,
    pub database_url: String,
    pub local_storage_path: String,
    pub host: Option<String>,
    pub port: Option<usize>,
    pub assets: Option<String>,
}

impl Config {
    pub fn new() -> Result<Self, Error> {
        envy::from_env()
    }

    pub fn address(&self) -> String {
        let default_host = "0.0.0.0".to_owned();
        let host = self.host.as_ref().unwrap_or(&default_host);
        let port = self.port.unwrap_or(4200);
        format!("{}:{}", host, port)
    }

    pub fn assets(&self) -> String {
        match &self.assets {
            Some(assets) => assets.clone(),
            None => "./files-store-web/public".to_string(),
        }
    }
}
