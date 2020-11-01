use actix_web::{
    get, post, put,
    web::{Data, HttpResponse, Json, ServiceConfig},
};
use sqlx::PgPool;
use users::domain::{ChangePassword, PasswordCredentials, Signup, User, UserUpdate, UsersStore};
use users::users_service;
use validator::Validate;

use crate::config::Config;
use crate::domain::{FsNodeMetadata, FsNodeType};
use crate::errors::ApiError;
use crate::repositories::FsNodeStore;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(login_service);
    cfg.service(signup_service);
    cfg.service(me_service);
    cfg.service(change_password_service);
    cfg.service(update_user_service);
}

#[post("/api/users/login")]
pub async fn login_service(
    payload: Json<PasswordCredentials>,
    config: Data<Config>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, ApiError> {
    payload.validate()?;
    let mut connection = pool.acquire().await?;
    let access_token =
        users_service::login(&config.into_inner().secret_key, &payload, &mut connection).await?;
    Ok(HttpResponse::Ok().json(access_token))
}

#[post("/api/users/signup")]
pub async fn signup_service(
    payload: Json<Signup>,
    pool: Data<PgPool>,
    config: Data<Config>,
) -> Result<HttpResponse, ApiError> {
    payload.validate()?;
    let mut transaction = pool.begin().await?;
    let (access_token, user) = users_service::signup(
        &config.into_inner().secret_key,
        payload.into_inner(),
        &mut transaction,
    )
    .await?;
    transaction
        .insert_root_fs_node(&FsNodeType::Root, "Root", &FsNodeMetadata::Root, &user.uuid)
        .await?;
    transaction
        .insert_root_fs_node(&FsNodeType::Bin, "Bin", &FsNodeMetadata::Bin, &user.uuid)
        .await?;
    transaction.commit().await?;
    Ok(HttpResponse::Ok().json(access_token))
}

#[get("/api/users/me")]
pub fn me_service(user: User) -> HttpResponse {
    HttpResponse::Ok().json(user)
}

#[put("/api/users")]
pub async fn update_user_service(
    payload: Json<UserUpdate>,
    pool: Data<PgPool>,
    user: User,
) -> Result<HttpResponse, ApiError> {
    payload.validate()?;
    let mut connection = pool.acquire().await?;
    let user = connection.update_user(&user.uuid, &payload).await?;
    Ok(HttpResponse::Ok().json(user))
}

#[put("/api/users/password")]
pub async fn change_password_service(
    payload: Json<ChangePassword>,
    pool: Data<PgPool>,
    user: User,
) -> Result<HttpResponse, ApiError> {
    payload.validate()?;
    let mut transaction = pool.begin().await?;
    let user = users_service::change_password(&payload, &user, &mut transaction).await?;
    transaction.commit().await?;
    Ok(HttpResponse::Ok().json(user))
}
