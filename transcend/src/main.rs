use axum::{
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use regex::Regex;
use bcrypt::{hash, DEFAULT_COST};
use sqlx::PgPool;

#[derive(Deserialize)]
struct RegisterRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct ApiResponse {
    success: bool,
    message: String,
}

#[derive(Debug)]
struct User {
    email: String,
    password_hash: String,
}

static USERS: Lazy<Mutex<Vec<User>>> = Lazy::new(|| Mutex::new(Vec::new()));

async fn register_user(
    Json(payload): Json<RegisterRequest>,
) -> Json<ApiResponse> {
    // Email validation
    let email_regex = Regex::new(r"^\S+@\S+\.\S+$").unwrap();
    if !email_regex.is_match(&payload.email) {
        return Json(ApiResponse {
            success: false,
            message: "Invalid email format".into(),
        });
    }

    if payload.password.len() < 6 {
        return Json(ApiResponse {
            success: false,
            message: "Password must be at least 6 characters".into(),
        });
    }

    let mut users = USERS.lock().unwrap();

    if users.iter().any(|u| u.email == payload.email) {
        return Json(ApiResponse {
            success: false,
            message: "User already exists".into(),
        });
    }

    let password_hash = hash(&payload.password, DEFAULT_COST).unwrap();

    users.push(User {
        email: payload.email,
        password_hash,
    });

    Json(ApiResponse {
        success: true,
        message: "User registered successfully".into(),
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/register", post(register_user));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
