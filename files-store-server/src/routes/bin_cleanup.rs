use actix_web::{
    delete,
    web::{Data, HttpResponse},
};
use users::domain::User;

use crate::errors::ApiError;
use crate::jobs::bin_cleaner_job::{BinCleanerActorAddr, Cleanup};

#[delete("/api/files/cleanup")]
async fn bin_cleanup_route(
    bin_cleanup_job: Data<BinCleanerActorAddr>,
    user: User,
) -> Result<HttpResponse, ApiError> {
    tracing::info!("Cleanup bin request user_uuid={}", user.uuid);
    let _ = bin_cleanup_job.send(Cleanup::new(user.uuid)).await?;
    Ok(HttpResponse::Ok().finish())
}
