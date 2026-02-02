pub mod chat_hub;
pub mod chat_server;
pub mod messages;

use actix_web::{web, HttpRequest, HttpResponse, Error};
use actix_web_actors::ws;
use uuid::Uuid;
use actix::Actor;
use actix_web::web::Data;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use actix::Addr;
use sqlx::PgPool;
use crate::utils::jwt::Claims;

use chat_hub::ChatSocket;
use chat_server::ChatServer;

pub fn config(cfg: &mut web::ServiceConfig, pool: web::Data<sqlx::PgPool>) {
    let server = ChatServer::new().start();

    cfg.app_data(web::Data::new(server));
    cfg.app_data(pool.clone()); // <-- make pool available to ws handler

    cfg.service(
        web::resource("/hubs/chat")
            .route(web::get().to(chat_ws))
    );
}

async fn chat_ws(
    req: HttpRequest,
    stream: web::Payload,
    server: web::Data<Addr<ChatServer>>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("No token"))?;
    let secret = std::env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set");

    // Decode JWT and extract user_id
    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256)
    ).map_err(|_| actix_web::error::ErrorUnauthorized("Invalid token"))?;

    let user_id = claims.claims.sub;

    let socket = ChatSocket {
        user_id,
        server: server.get_ref().clone(),
        pool: pool.get_ref().clone(),
    };

    ws::start(socket, &req, stream)
}
