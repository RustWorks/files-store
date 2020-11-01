use bcrypt::BcryptError;
use serde::Deserialize;
use std::convert::TryFrom;
use validator::Validate;

use crate::domain::User;
use crate::password::secure_user;

#[derive(Debug, Clone, Validate, Deserialize)]
pub struct Signup {
    #[validate(length(min = 3, message = "validation.login.short"))]
    pub login: String,
    #[validate(email(message = "validation.email"))]
    pub email: String,
    #[validate(length(min = 6, message = "validation.password.short"))]
    pub password: String,
}

impl TryFrom<Signup> for User {
    type Error = BcryptError;

    fn try_from(signup: Signup) -> Result<Self, Self::Error> {
        let user = secure_user(signup.login, signup.email, signup.password)?;
        Ok(user)
    }
}
