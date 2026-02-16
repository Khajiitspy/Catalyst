use actix_web::{put, post, web, HttpResponse, HttpRequest};
use validator::Validate;
use sqlx::PgPool;
use actix_multipart::Multipart;
use uuid::Uuid;
use futures_util::{StreamExt, TryStreamExt};
use log::{error};

use crate::{
    models::user::{LoginUser, EditProfileRequest, AuthResponse, User},
    services::image_service::save_image_variants,
    utils::{errors::ApiError, jwt, password::hash_password},
    db::user_repository::UserRepository,
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
    let mut image: Option<String> = None;

    println!("➡️ Parsing multipart...");
    while let Some(field_result) = payload.next().await {
        let mut field = field_result.map_err(|_| ApiError::InternalServerError)?;
        let cd = field.content_disposition().unwrap();
        let name = cd.get_name().unwrap_or_default().to_string();
        let filename = cd.get_filename().map(|s| s.to_string());

        println!("📦 Field: {} Filename: {:?}", name, filename);

        if name == "imageFile" {
            // let original_filename =
            //     filename.ok_or(ApiError::ValidationError("No filename".into()))?;

            let base_name = format!("{}.webp", Uuid::new_v4());

            let mut bytes = Vec::new();

            while let Some(chunk_res) = field.next().await {
                let chunk = chunk_res
                    .map_err(|_| ApiError::InternalServerError)?;
                bytes.extend_from_slice(&chunk);
            }


            save_image_variants(&bytes, &base_name)
                .map_err(|_| ApiError::InternalServerError)?;

            image = Some(base_name);
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
        image
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
        image.as_deref(),
    )
    .await
    .map_err(|e| {
        println!("❌ DB ERROR: {:?}", e);
        ApiError::InternalServerError
    })?;

    println!("🔐 Creating JWT...");

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token = jwt::create_token(&user, &secret)
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
    let token = jwt::create_token(&user, &secret)
        .map_err(|_| ApiError::InternalServerError)?;

    Ok(HttpResponse::Ok().json(AuthResponse { token }))
}

#[put("/Auth/profile")]
pub async fn edit_profile(
    pool: web::Data<PgPool>,
    mut payload: Multipart,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    println!("👉 HIT /Auth/profile");

    // --------------------------------------------------------------
    // 1️⃣  Authenticate – extract user_id from the JWT
    // --------------------------------------------------------------
    let auth_header = req
        .headers()
        .get(actix_web::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(ApiError::Unauthorized)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(ApiError::Unauthorized)?;

    let secret = std::env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set");
    let claims = jwt::decode_token(token, &secret)
        .map_err(|_| ApiError::Unauthorized)?;
    let user_id = claims.sub; // adjust if your claim uses a different field name

    // --------------------------------------------------------------
    // 2️⃣  Parse multipart body
    // --------------------------------------------------------------
    let mut edit_req = EditProfileRequest {
        first_name: None,
        last_name: None,
        email: None,
    };
    let mut new_image_name: Option<String> = None;

    while let Some(field_res) = payload.next().await {
        let mut field = field_res.map_err(|_| ApiError::InternalServerError)?;

        // // ----- grab disposition and field name (immutable borrow) -----
        // let disposition = field
        //     .content_disposition()
        //     .ok_or(ApiError::ValidationError(
        //         "Missing content disposition".into(),
        //     ))?;
        let cd = field.content_disposition().unwrap();
        let name = cd.get_name().unwrap_or_default().to_string();
        // let name = disposition.get_name().unwrap_or_default().to_string();
        let filename = cd.get_filename().map(|s| s.to_string());

        println!("📦 Field: {} Filename: {:?}", name, filename);

        if name == "imageFile" {
            let base_name = format!("{}.webp", Uuid::new_v4());
            let mut bytes = Vec::new();
            println!("📦 image new name: {}", base_name);

            // Read the whole file into `bytes`
            while let Some(chunk_res) = field.next().await {
                let chunk = chunk_res
                    .map_err(|_| ApiError::InternalServerError)?;
                bytes.extend_from_slice(&chunk);
            }

            save_image_variants(&bytes, &base_name)
                .map_err(|_| ApiError::InternalServerError)?;
            new_image_name = Some(base_name);
        }
        // --------------------------------------------------------------
        // Text fields
        // --------------------------------------------------------------
        else {
            let mut data = Vec::new();

            while let Some(chunk_res) = field.next().await {
                let chunk = chunk_res
                    .map_err(|_| ApiError::InternalServerError)?;
                data.extend_from_slice(&chunk);
            }

            let value = String::from_utf8(data)
                .map_err(|_| ApiError::ValidationError("Invalid UTF‑8".into()))?;

            match name.as_str() {
                "firstName" => edit_req.first_name = Some(value),
                "lastName" => edit_req.last_name = Some(value),
                "email" => edit_req.email = Some(value),
                _ => {} // ignore unknown fields
            }
        }
    }

    println!(
        "✅ Parsed: first_name={:?}, last_name={:?}, email={:?}, image={:?}",
        edit_req.first_name,
        edit_req.last_name,
        edit_req.email,
        new_image_name
    );

    // --------------------------------------------------------------
    // 3️⃣  Validate the incoming fields (only those that are Some)
    // --------------------------------------------------------------
    edit_req
        .validate()
        .map_err(|e| ApiError::ValidationError(e.to_string()))?;

    // --------------------------------------------------------------
    // 4️⃣  Persist the changes
    // --------------------------------------------------------------
    let repo = UserRepository::new(pool.get_ref().clone());

    let updated_user = repo
        .update_user(user_id, edit_req, new_image_name.as_deref())
        .await
        .map_err(|e| {
            error!("DB error while updating profile: {:?}", e);
            ApiError::InternalServerError
        })?;

    // --------------------------------------------------------------
    // 5️⃣  Create a fresh JWT for the *updated* user
    // --------------------------------------------------------------
    let new_token = jwt::create_token(&updated_user, &secret)
        .map_err(|_| ApiError::InternalServerError)?;

    Ok(HttpResponse::Ok().json(AuthResponse {
        token: new_token,
    }))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(register)
       .service(login)
       .service(edit_profile);
}
