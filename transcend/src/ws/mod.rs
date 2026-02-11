pub mod chat_hub;
pub mod chat_server;
pub mod messages;

use actix::{Actor, Addr};
use actix_web::{web, HttpRequest, HttpResponse, Error};
use actix_web_actors::ws;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use sqlx::PgPool;

use crate::utils::jwt::Claims;
use crate::ws::chat_server::ChatServer;
use chat_hub::ChatSocket;

pub fn config(cfg: &mut web::ServiceConfig, pool: web::Data<PgPool>) {
    let server = ChatServer::new().start();

    cfg.app_data(web::Data::new(server));
    cfg.app_data(pool.clone());

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
    // 🔐 Extract Bearer token
    // let token = req
    //     .headers()
    //     .get("Authorization")
    //     .and_then(|h| h.to_str().ok())
    //     .and_then(|s| s.strip_prefix("Bearer "))
    //     .ok_or_else(|| actix_web::error::ErrorUnauthorized("No token"))?;
    let token = req
        .query_string()
        .split('&')
        .find_map(|pair| {
            let mut parts = pair.split('=');
            match (parts.next(), parts.next()) {
                (Some("token"), Some(val)) => Some(val),
                _ => None,
            }
        })
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("No token"))?;


    let secret = std::env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set");

    // 🔓 Decode JWT
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    ).map_err(|_| actix_web::error::ErrorUnauthorized("Invalid token"))?;

    let user_id = token_data.claims.sub;

    // 🔌 Create socket
    let socket = ChatSocket {
        user_id,
        server: server.get_ref().clone(),
        pool: pool.get_ref().clone(),
    };

    ws::start(socket, &req, stream)
}
