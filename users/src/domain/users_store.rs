use async_trait::async_trait;
use sqlx::Error;
use uuid::Uuid;

use crate::domain::{User, UserUpdate};

#[async_trait]
pub trait UsersStore {
    async fn insert_user(&mut self, user: &User) -> Result<User, Error>;
    async fn find_users_by_uuid(&mut self, uuid: &Uuid) -> Result<User, Error>;
    async fn find_users_by_email(&mut self, email: &str) -> Result<Option<User>, Error>;
    async fn update_user(
        &mut self,
        user_uuid: &Uuid,
        user_update: &UserUpdate,
    ) -> Result<User, Error>;
    async fn update_user_password(
        &mut self,
        user_uuid: &Uuid,
        password: &str,
    ) -> Result<User, Error>;
}
