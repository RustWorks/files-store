use std::convert::TryFrom;

use sqlx::PgConnection;

use crate::domain::{AccessToken, ChangePassword, PasswordCredentials, Signup, User, UsersStore};
use crate::errors::Error;
use crate::jwt::create_token;
use crate::password::{hash_password, verify_password};

pub async fn login(
    secret_key: &str,
    password_credentials: &PasswordCredentials,
    pg_connection: &mut PgConnection,
) -> Result<AccessToken, Error> {
    let user = pg_connection
        .find_users_by_email(&password_credentials.username.trim().to_lowercase())
        .await?
        .ok_or(Error::Unauthorized)?;
    if let Ok(true) = verify_password(&password_credentials.password, &user.password) {
        let (token, claime) = create_token(user, secret_key)?;
        Ok(AccessToken::new(token, claime.exp))
    } else {
        Err(Error::Unauthorized)
    }
}

pub async fn change_password(
    change_password: &ChangePassword,
    user: &User,
    pg_connection: &mut PgConnection,
) -> Result<User, Error> {
    let user = pg_connection
        .find_users_by_email(&user.email.trim().to_lowercase())
        .await?
        .ok_or(Error::NotFound)?;
    if let Ok(true) = verify_password(&change_password.old_password, &user.password) {
        let new_password = hash_password(&change_password.new_password)?;
        let updated_user = pg_connection
            .update_user_password(&user.uuid, &new_password)
            .await?;
        Ok(updated_user)
    } else {
        Err(Error::Invalid {
            message: "Old password invalid".to_owned(),
        })
    }
}

pub async fn signup(
    secret_key: &str,
    payload: Signup,
    pg_connection: &mut PgConnection,
) -> Result<(AccessToken, User), Error> {
    let user = User::try_from(payload)?;
    let user = pg_connection.insert_user(&user).await?;
    let (token, claime) = create_token(user.clone(), secret_key)?;
    let access_token = AccessToken::new(token, claime.exp);
    Ok((access_token, user))
}
