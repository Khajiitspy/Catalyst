use chrono::{Utc, Duration};
use jsonwebtoken::{EncodingKey, DecodingKey, Header, Validation, encode, decode, TokenData, errors::Result};
use serde::{Serialize, Deserialize};
use crate::models::user::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub name: String,
    pub email: String,
    pub image: Option<String>,
    pub roles: Vec<String>,
    pub exp: usize,
}

pub fn create_token(user: &User, secret: &str) -> Result<String> {
    // let expiration = Utc::now()
    //     .checked_add_signed(Duration::hours(24))
    //     .expect("valid timestamp")
    //     .timestamp() as usize;

    let claims = Claims {
        sub: user.id,
        name: format!("{} {}", user.first_name, user.last_name),
        email: user.email.clone(),
        image: user.image.clone(),
        roles: vec!["user".to_string()],
        exp: (Utc::now() + Duration::hours(24)).timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;

    Ok(token)
}

pub fn decode_token(
    token: &str,
    secret: &str,
) -> jsonwebtoken::errors::Result<Claims> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;

    println!("Decoded token claims: {:?}", data.claims);

    Ok(data.claims)
}


pub fn verify_token(token: &str, secret: &str) -> Result<TokenData<Claims>> {
    decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default())
}
