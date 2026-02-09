use actix_web::{FromRequest, HttpRequest};
use futures_util::future::{ready, Ready};

use crate::utils::errors::ApiError;
use crate::utils::jwt::decode_token;

pub struct AuthUser {
    pub user_id: i64,
}

impl FromRequest for AuthUser {
    type Error = ApiError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let header = match req.headers().get("Authorization") {
            Some(h) => h.to_str().ok(),
            None => None,
        };

        let token = match header.and_then(|h| h.strip_prefix("Bearer ")) {
            Some(t) => t,
            None => return ready(Err(ApiError::Unauthorized)),
        };

        let secret = std::env::var("JWT_SECRET").unwrap();
        let claims = decode_token(token, &secret)
            .map_err(|_| ApiError::Unauthorized);

        match claims {
            Ok(c) => ready(Ok(AuthUser { user_id: c.sub })),
            Err(e) => ready(Err(e)),
        }
    }
}
