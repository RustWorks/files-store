use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Token(pub String);

#[derive(Debug, Serialize)]
pub struct AccessToken {
    pub access_token: Token,
    pub token_type: String,
    pub expires_in: i64,
}

impl AccessToken {
    pub fn new(access_token: Token, expires_in: i64) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_owned(),
            expires_in,
        }
    }
}
