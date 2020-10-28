use actix_web::{http, HttpResponse, ResponseError};
use serde_json::json;
use std::borrow::Cow;
use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Internal Server Error")]
    InternalServer,
    #[error("invalid")]
    Invalid { message: String },
    #[error("duplicate")]
    Duplicate,
    #[error("NotFound")]
    NotFound,
    #[error("io")]
    IO(#[from] std::io::Error),
    #[error("sqlx")]
    Sqlx(#[from] sqlx::Error),
    #[error("serde_json")]
    SerdeJson(#[from] serde_json::Error),
    #[error("jobs")]
    Jobs(#[from] actix::MailboxError),
    #[error("blocking_error")]
    BlockingError(#[from] actix_web::error::BlockingError<image::ImageError>),
    #[error("image error")]
    ImageError(#[from] image::ImageError),
    #[error("strum parse")]
    Strum(#[from] strum::ParseError),
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        tracing::error!("Api error: {}", &self);
        match self {
            Self::Invalid { message } => HttpResponse::build(http::StatusCode::BAD_REQUEST)
                .json(json!({ "message": message })),
            Self::Duplicate => HttpResponse::build(http::StatusCode::BAD_REQUEST)
                .json(json!({ "message": "already.exists" })),
            Self::NotFound | Self::Sqlx(sqlx::Error::RowNotFound) => {
                HttpResponse::build(http::StatusCode::NOT_FOUND)
                    .json(json!({ "message": "not.found" }))
            }
            Self::Sqlx(sqlx::Error::Database(e)) if e.code() == Some(Cow::from("23505")) => {
                HttpResponse::build(http::StatusCode::BAD_REQUEST)
                    .json(json!({ "message": "already.exists" }))
            }
            Self::InternalServer
            | Self::IO(_)
            | Self::Sqlx(_)
            | Self::Jobs(_)
            | Self::BlockingError(_)
            | Self::ImageError(_)
            | Self::Strum(_)
            | Self::SerdeJson(_) => HttpResponse::build(http::StatusCode::INTERNAL_SERVER_ERROR)
                .json(json!({ "message": "technical.error" })),
        }
    }
}
