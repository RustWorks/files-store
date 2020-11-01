use async_trait::async_trait;
use sqlx::{query_as, Error, PgConnection};
use uuid::Uuid;

use crate::domain::{User, UserUpdate, UsersStore};

#[async_trait]
impl UsersStore for PgConnection {
    async fn insert_user(&mut self, user: &User) -> Result<User, Error> {
        let user = query_as(
            r#"
            INSERT INTO users (
                uuid,
                login,
                email,
                password,
                created_at,
                updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6) RETURNING *
            "#,
        )
        .bind(user.uuid)
        .bind(&user.login)
        .bind(&user.email)
        .bind(&user.password)
        .bind(user.created_at)
        .bind(user.updated_at)
        .fetch_one(self)
        .await?;
        Ok(user)
    }

    async fn find_users_by_uuid(&mut self, uuid: &Uuid) -> Result<User, Error> {
        let user = query_as("SELECT * FROM users WHERE uuid = $1")
            .bind(uuid)
            .fetch_one(self)
            .await?;
        Ok(user)
    }

    async fn find_users_by_email(&mut self, email: &str) -> Result<Option<User>, Error> {
        let user = query_as("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(self)
            .await?;
        Ok(user)
    }

    async fn update_user(
        &mut self,
        user_uuid: &Uuid,
        user_update: &UserUpdate,
    ) -> Result<User, Error> {
        let user = query_as("UPDATE users SET login = $1, email = $2 WHERE uuid = $3 RETURNING *")
            .bind(&user_update.login)
            .bind(&user_update.email)
            .bind(user_uuid)
            .fetch_one(self)
            .await?;
        Ok(user)
    }

    async fn update_user_password(
        &mut self,
        user_uuid: &Uuid,
        password: &str,
    ) -> Result<User, Error> {
        let user = query_as("UPDATE users SET password = $1 WHERE uuid = $2 RETURNING *")
            .bind(password)
            .bind(user_uuid)
            .fetch_one(self)
            .await?;
        Ok(user)
    }
}
