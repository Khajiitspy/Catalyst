use actix_web::{put, get, post, web, HttpResponse};
use validator::Validate;
use sqlx::PgPool;
use crate::services::chat_service::ChatService;

use crate::{
    models::chat::{
        ChatCreateModel,
        ChatEditModel,
        UserSearchModel,
        SendMessageModel,
    },

    utils::{auth::AuthUser, errors::ApiError},
};

#[post("/chats")]
pub async fn create_chat(
    pool: web::Data<PgPool>,
    user: AuthUser,
    model: web::Json<ChatCreateModel>,
) -> Result<HttpResponse, ApiError> {
    println!("Creating chat: {:?}", model);
    let service = ChatService::new(pool.get_ref().clone());
    let chat_id = service.create_chat(model.into_inner(), user.user_id).await?;
    Ok(HttpResponse::Ok().json(chat_id))
}

#[get("/chats/my")]
pub async fn get_user_chats(
    pool: web::Data<PgPool>,
    user: AuthUser,
) -> Result<HttpResponse, ApiError> {
    let service = ChatService::new(pool.get_ref().clone());
    let chats = service.get_user_chats(user.user_id).await?;
    Ok(HttpResponse::Ok().json(chats))
}

#[get("/chats/{chat_id}/messages")]
pub async fn get_chat_messages(
    pool: web::Data<PgPool>,
    user: AuthUser,
    path: web::Path<i64>,
) -> Result<HttpResponse, ApiError> {
    let service = ChatService::new(pool.get_ref().clone());
    let messages = service.get_chat_messages(*path, user.user_id).await?;
    Ok(HttpResponse::Ok().json(messages))
}

#[get("/chats/types")]
pub async fn get_chat_types(
    pool: web::Data<PgPool>,
    _user: AuthUser,
) -> Result<HttpResponse, ApiError> {
    let service = ChatService::new(pool.get_ref().clone());

    let types = service
        .get_chat_types()
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    println!("📦 Types:\n{:#?}", types);

    Ok(HttpResponse::Ok().json(types))
}

#[post("/chats/messages")]
pub async fn send_message(
    pool: web::Data<PgPool>,
    user: AuthUser,
    payload: web::Json<SendMessageModel>,
) -> Result<HttpResponse, ApiError> {
    payload.validate()
        .map_err(|e: validator::ValidationErrors| ApiError::ValidationError(e.to_string()))?;

    let service = ChatService::new(pool.get_ref().clone());

    let message = service
        .send_message(payload.chat_id, user.user_id, payload.message.clone())
        .await
        .map_err(|_| ApiError::Unauthorized)?;

    Ok(HttpResponse::Ok().json(message))
}


#[put("/chats/edit")]
pub async fn edit_chat(
    pool: web::Data<PgPool>,
    user: AuthUser,
    payload: web::Json<ChatEditModel>,
) -> Result<HttpResponse, ApiError> {
    payload.validate()
        .map_err(|e| ApiError::ValidationError(e.to_string()))?;

    let service = ChatService::new(pool.get_ref().clone());
    service.edit_chat(payload.into_inner(), user.user_id).await?;

    Ok(HttpResponse::Ok().finish())
}

#[get("/chats/am-i-admin")]
pub async fn am_i_admin(
    pool: web::Data<PgPool>,
    user: AuthUser,
    query: web::Query<std::collections::HashMap<String, i64>>,
) -> Result<HttpResponse, ApiError> {
    let chat_id = *query.get("chatId")
        .ok_or(ApiError::ValidationError("chatId missing".into()))?;

    let service = ChatService::new(pool.get_ref().clone());
    let is_admin = service.am_i_admin(chat_id, user.user_id).await?;

    Ok(HttpResponse::Ok().json(is_admin))
}

#[get("/chats/users")]
pub async fn search_users(
    pool: web::Data<PgPool>,
    _user: AuthUser,
    query: web::Query<UserSearchModel>,
) -> Result<HttpResponse, ApiError> {
    let service = ChatService::new(pool.get_ref().clone());
    let users = service.search_users(query.into_inner()).await?;
    Ok(HttpResponse::Ok().json(users))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_chat)
        .service(get_chat_types)
        .service(send_message)
        .service(get_chat_messages)
        .service(get_user_chats)
        .service(edit_chat)
        .service(am_i_admin)
        .service(search_users);
}
