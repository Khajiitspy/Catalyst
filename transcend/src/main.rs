use actix_web::{App, HttpServer, web};
use db::pool::create_pool;

mod db;
mod handlers;
mod models;
mod utils;
mod ws;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().expect("Failed to load .env");

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = create_pool(&database_url)
        .await
        .expect("Failed to connect to database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))

            // 🔹 REST API
            .service(
                web::scope("/api")
                    .configure(handlers::users::config)
                    .configure(handlers::chats::config)
            )

            // 🔹 WebSocket / SignalR-style hub
            .configure(|cfg| ws::config(cfg, web::Data::new(pool.clone())))
    })
    .bind(("0.0.0.0", 5240))?
    .run()
    .await
}
