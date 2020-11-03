use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum AppError {
    #[error("io")]
    IO(#[from] std::io::Error),
}
