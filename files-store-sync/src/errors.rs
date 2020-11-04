use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum AppError {
    #[error("api")]
    Api,
    #[error("io")]
    IO(#[from] std::io::Error),
    #[error("reqwest")]
    Reqwest(#[from] reqwest::Error),
}
