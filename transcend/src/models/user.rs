use serde::{Serialize, Deserialize};
use validator::Validate;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RegisterUser {
    #[validate(length(min = 1, message = "First name is required"))]
    pub first_name: String,
    #[validate(length(min = 1, message = "Last name is required"))]
    pub last_name: String,
    #[validate(email(message = "Email must be valid"))]
    pub email: String,
    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginUser {
    #[validate(email(message = "Email must be valid"))]
    pub email: String,
    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: String,
    pub image: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct EditProfileRequest {
    #[validate(length(min = 1, message = "First name is required"))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    #[validate(length(min = 1, message = "Last name is required"))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    #[validate(email(message = "Email must be valid"))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}
