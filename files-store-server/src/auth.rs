use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use futures::future::{ok, Ready};
use uuid::Uuid;

use crate::errors::ApiError;

#[derive(Debug)]
pub struct User {
    pub uuid: Uuid,
}

impl User {
    pub fn fake() -> Self {
        Self {
            uuid: Uuid::parse_str("55e907b7-33ff-4989-a609-0b812c77daf6").unwrap(),
        }
    }
}

impl FromRequest for User {
    type Error = ApiError;
    type Config = ();
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(_: &HttpRequest, _: &mut Payload) -> Self::Future {
        ok(User::fake())
    }
}
