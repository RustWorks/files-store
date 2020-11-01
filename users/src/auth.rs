use actix_web::dev::Payload;
use actix_web::http::header;
use actix_web::web::Data;
use actix_web::{FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};

use crate::domain::User;
use crate::jwt::{decode_token, Claime};
use crate::Error;

#[derive(Debug)]
pub struct AuthConfig {
    pub secret_key: String,
}

impl AuthConfig {
    pub fn new(secret_key: String) -> Self {
        Self { secret_key }
    }
}

impl FromRequest for User {
    type Error = Error;
    type Config = ();
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        if let Some(auth_config) = req.app_data::<Data<AuthConfig>>() {
            extract_auth_claim(req, auth_config)
                .map(|auth| ok(auth.user))
                .unwrap_or_else(err)
        } else {
            err(Error::Unauthorized)
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct AuthQuery {
    access_token: String,
}

fn extract_from_header(req: &HttpRequest) -> Option<String> {
    req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|token| token.to_str().ok())
        .and_then(|header_value| {
            header_value
                .split_whitespace()
                .skip(1)
                .collect::<Vec<&str>>()
                .first()
                .map(|token| token.to_string())
        })
}

fn extract_from_query_string(req: &HttpRequest) -> Option<String> {
    serde_urlencoded::from_str::<AuthQuery>(req.query_string())
        .ok()
        .map(|query| query.access_token)
}

fn extract_auth_claim(req: &HttpRequest, auth_config: &AuthConfig) -> Result<Claime, Error> {
    extract_from_header(req)
        .or(extract_from_query_string(req))
        .ok_or(Error::Unauthorized)
        .and_then(|token| decode_token(&token, &auth_config.secret_key))
}
