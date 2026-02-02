use actix_web::{get, post, web, HttpResponse};
use validator::Validate;
use sqlx::PgPool;

use crate::{
    db::chat_repository::ChatRepository,
    models::chat::{ChatCreateModel, SendMessageModel},
    utils::{auth::AuthUser, errors::ApiError},
};

#[post("/chats")]
pub async fn create_chat(
    pool: web::Data<PgPool>,
    user: AuthUser,
    payload: web::Json<ChatCreateModel>,
) -> Result<HttpResponse, ApiError> {
    payload.validate()
        .map_err(|e| ApiError::ValidationError(e.to_string()))?;

    let repo = ChatRepository::new(pool.get_ref().clone());

    let chat_id = repo
        .create_chat(&payload, user.user_id)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    Ok(HttpResponse::Ok().json(chat_id))
}

#[get("/chats/types")]
pub async fn get_chat_types(
    pool: web::Data<PgPool>,
    _user: AuthUser,
) -> Result<HttpResponse, ApiError> {
    let repo = ChatRepository::new(pool.get_ref().clone());

    let types = repo
        .get_chat_types()
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    Ok(HttpResponse::Ok().json(types))
}

#[post("/chats/messages")]
pub async fn send_message(
    pool: web::Data<PgPool>,
    user: AuthUser,
    payload: web::Json<SendMessageModel>,
) -> Result<HttpResponse, ApiError> {
    payload.validate()
        .map_err(|e| ApiError::ValidationError(e.to_string()))?;

    let repo = ChatRepository::new(pool.get_ref().clone());

    let message = repo
        .send_message(payload.chat_id, user.user_id, &payload.message)
        .await
        .map_err(|_| ApiError::Unauthorized)?;

    Ok(HttpResponse::Ok().json(message))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_chat)
        .service(get_chat_types)
        .service(send_message);
}
