use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct ChangePassword {
    #[validate(length(min = 6, message = "validation.password.short"))]
    pub old_password: String,
    #[validate(length(min = 6, message = "validation.password.short"))]
    pub new_password: String,
}
