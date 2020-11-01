use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Clone, Validate, Deserialize)]
pub struct PasswordCredentials {
    #[validate(contains = "password")]
    pub grant_type: String,
    #[validate(email(message = "validation.email"))]
    pub username: String,
    #[validate(length(min = 6, message = "validation.password.short"))]
    pub password: String,
    pub scope: Option<String>,
}
