use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct UserUpdate {
    #[validate(
        length(min = 3, message = "validation.login.short"),
        length(max = 10, message = "validation.login.long")
    )]
    pub login: String,
    #[validate(email(message = "validation.email"))]
    pub email: String,
}
