use chrono::{Utc, Duration};
use jsonwebtoken::{EncodingKey, DecodingKey, Header, Validation, encode, decode, TokenData, errors::Result};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,          // user ID
    pub exp: usize,         // expiration timestamp
}

pub fn create_token(user_id: Uuid, secret: &str) -> Result<String> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        exp: expiration,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
}

pub fn verify_token(token: &str, secret: &str) -> Result<TokenData<Claims>> {
    decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default())
}
