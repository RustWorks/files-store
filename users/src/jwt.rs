use chrono::{Duration, Utc};
use derive_new::new;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::ops::Add;
use uuid::Uuid;

use crate::domain::{Token, User};
use crate::Error;

#[derive(Debug, new, Serialize, Deserialize)]
pub struct Claime {
    pub sub: Uuid,
    pub exp: i64,
    pub user: User,
}

pub fn create_token(user: User, secret_key: &str) -> Result<(Token, Claime), Error> {
    let claims = Claime::new(
        user.uuid,
        Utc::now().add(Duration::days(30)).timestamp(),
        user,
    );
    let token = encode(
        &Header::new(Algorithm::HS512),
        &claims,
        &EncodingKey::from_secret(secret_key.as_bytes()),
    )?;
    Ok((Token(token), claims))
}

pub fn decode_token(token: &str, secret_key: &str) -> Result<Claime, Error> {
    let validation = Validation {
        validate_exp: true,
        algorithms: vec![Algorithm::HS512],
        ..Validation::default()
    };
    let claims = decode::<Claime>(
        token,
        &DecodingKey::from_secret(secret_key.as_bytes()),
        &validation,
    )?;
    let claims = claims.claims;
    Ok(claims)
}
