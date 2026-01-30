use actix_web::{post, web, HttpResponse};
use validator::Validate;
use sqlx::PgPool;
use actix_multipart::Multipart;
use futures_util::StreamExt;
use std::io::Write;
use crate::utils::jwt;
use sanitize_filename::sanitize;

use crate::{
    db::user_repository::UserRepository,
    models::user::{LoginUser, AuthResponse},
    utils::{
        errors::ApiError,
        password::hash_password,
    },
};

#[post("/Auth/register")]
pub async fn register(
    pool: web::Data<PgPool>,
    mut payload: Multipart,
) -> Result<HttpResponse, ApiError> {
    println!("👉 HIT /Auth/register");

    let mut first_name: Option<String> = None;
    let mut last_name: Option<String> = None;
    let mut email: Option<String> = None;
    let mut password: Option<String> = None;
    let mut image_filename: Option<String> = None;

    println!("➡️ Parsing multipart...");
    while let Some(field_result) = payload.next().await {
        let mut field = field_result.map_err(|_| ApiError::InternalServerError)?;
        let cd = field.content_disposition().unwrap();
        let name = cd.get_name().unwrap_or_default().to_string();
        let filename = cd.get_filename().map(|s| s.to_string());

        println!("📦 Field: {} Filename: {:?}", name, filename);

        if name == "imageFile" {
            let filename = filename.ok_or(ApiError::ValidationError("No filename".into()))?;
            let safe_filename = sanitize(&filename);
            let filepath = format!("./uploads/{}", safe_filename);

            let mut f = std::fs::File::create(&filepath)
                .map_err(|_| ApiError::InternalServerError)?;

            while let Some(chunk) = field.next().await {
                let bytes = chunk.map_err(|_| ApiError::InternalServerError)?;
                f.write_all(&bytes).map_err(|_| ApiError::InternalServerError)?;
            }

            image_filename = Some(safe_filename);
        } else {
            let mut data = Vec::new();
            while let Some(chunk) = field.next().await {
                let bytes = chunk.map_err(|_| ApiError::InternalServerError)?;
                data.extend_from_slice(&bytes);
            }

            let value = String::from_utf8(data).map_err(|_| ApiError::InternalServerError)?;
            match name.as_str() {
                "firstName" => first_name = Some(value),
                "lastName" => last_name = Some(value),
                "email" => email = Some(value),
                "password" => password = Some(value),
                _ => {}
            }
        }
    }

    println!(
        "✅ Parsed: first_name={:?}, last_name={:?}, email={:?}, password_present={}, image={:?}",
        first_name,
        last_name,
        email,
        password.is_some(),
        image_filename
    );

    let first_name = first_name.ok_or(ApiError::ValidationError("firstName missing".into()))?;
    let last_name = last_name.ok_or(ApiError::ValidationError("lastName missing".into()))?;
    let email = email.ok_or(ApiError::ValidationError("email missing".into()))?;
    let password = password.ok_or(ApiError::ValidationError("password missing".into()))?;

    let register_user = crate::models::user::RegisterUser {
        first_name,
        last_name,
        email,
        password,
    };

    register_user.validate()
        .map_err(|e| ApiError::ValidationError(e.to_string()))?;

    let password_hash = hash_password(&register_user.password)
        .map_err(|_| ApiError::InternalServerError)?;

    let repo = UserRepository::new(pool.get_ref().clone());

    println!("💾 Creating user...");

    let user = repo.create_user(
        &register_user.first_name,
        &register_user.last_name,
        &register_user.email,
        &password_hash,
        image_filename.as_deref(),
    )
    .await
    .map_err(|e| {
        println!("❌ DB ERROR: {:?}", e);
        ApiError::InternalServerError
    })?;

    println!("🔐 Creating JWT...");

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token = jwt::create_token(user.id, &secret)
        .map_err(|_| ApiError::InternalServerError)?;

    Ok(HttpResponse::Ok().json(crate::models::user::AuthResponse { token }))
}

#[post("/Auth/login")]
pub async fn login(
    pool: web::Data<PgPool>,
    payload: web::Json<LoginUser>,
) -> Result<HttpResponse, ApiError> {
    payload.validate()
        .map_err(|e| ApiError::ValidationError(e.to_string()))?;

    let repo = UserRepository::new(pool.get_ref().clone());

    let user = repo
        .find_by_email(&payload.email)
        .await
        .map_err(|_| ApiError::InternalServerError)?
        .ok_or_else(|| ApiError::Unauthorized)?;

    println!("🧾 Login user fetched: {:?}", user);

    let valid = crate::utils::password::verify_password(
        &payload.password,
        &user.password_hash,
    )
    .map_err(|_| ApiError::InternalServerError)?;

    if !valid {
        return Err(ApiError::Unauthorized);
    }

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token = crate::utils::jwt::create_token(user.id, &secret)
        .map_err(|_| ApiError::InternalServerError)?;

    Ok(HttpResponse::Ok().json(AuthResponse { token }))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(register)
       .service(login);
}
