use actix_web::{HttpResponse, ResponseError};
use std::fmt;

#[derive(Debug)]
pub enum ApiError {
    ValidationError(String),
    Unauthorized,
    InternalServerError,
    BadRequest,
    // etc
}

// Implement Display (optional but good for logging)
impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            ApiError::Unauthorized => write!(f, "Unauthorized"),
            ApiError::InternalServerError => write!(f, "Internal server error"),
            ApiError::BadRequest => write!(f, "The request is invalid"),
        }
    }
}

// ✅ Implement ResponseError
impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        println!("❌ API ERROR: {:?}", self); // <-- logs all errors to console

        match self {
            ApiError::ValidationError(msg) =>
                HttpResponse::BadRequest().json(msg),
            ApiError::Unauthorized =>
                HttpResponse::Unauthorized().finish(),
            _ =>
                HttpResponse::InternalServerError().finish(),
        }
    }
}
