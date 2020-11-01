use actix_web::{http, HttpResponse, ResponseError};
use serde_json::json;
use std::borrow::Cow;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("unauthorized")]
    Unauthorized,
    #[error("invalid")]
    Invalid { message: String },
    #[error("already exist")]
    Duplicate,
    #[error("database")]
    Database(#[from] sqlx::Error),
    #[error("not found")]
    NotFound,
    #[error("token")]
    Token(#[from] jsonwebtoken::errors::Error),
    #[error("bcrypt")]
    Bcrypt(#[from] bcrypt::BcryptError),
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        log::error!("Users error {:#?}", self);
        match self {
            Self::Invalid { message } => HttpResponse::build(http::StatusCode::BAD_REQUEST)
                .json(json!({ "message": message })),
            Self::Duplicate => HttpResponse::build(http::StatusCode::BAD_REQUEST)
                .json(json!({ "message": "category.already.exists" })),
            Self::NotFound => HttpResponse::build(http::StatusCode::NOT_FOUND)
                .json(json!({ "message": "not.found" })),

            Self::Database(sqlx::Error::RowNotFound) => {
                HttpResponse::build(http::StatusCode::NOT_FOUND)
                    .json(json!({ "message": "not.found" }))
            }
            Self::Database(sqlx::Error::Database(e)) if e.code() == Some(Cow::from("23505")) => {
                HttpResponse::build(http::StatusCode::BAD_REQUEST)
                    .json(json!({ "message": "category.already.exists" }))
            }
            Self::Database(_) => HttpResponse::build(http::StatusCode::INTERNAL_SERVER_ERROR)
                .json(json!({ "message": "technical.error" })),
            Self::Unauthorized | Self::Token(_) | Self::Bcrypt(_) => {
                HttpResponse::build(http::StatusCode::UNAUTHORIZED)
                    .json(json!({ "message": "unauthorized" }))
            }
        }
    }
}
