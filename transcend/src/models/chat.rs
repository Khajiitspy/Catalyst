use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ChatTypeItemModel {
    pub id: Uuid,
    pub type_name: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ChatCreateModel {
    #[validate(length(min = 1))]
    pub name: Option<String>,
    pub chat_type_id: Uuid,
}

#[derive(Debug, Deserialize, Validate)]
pub struct SendMessageModel {
    pub chat_id: Uuid,

    #[validate(length(min = 1, max = 1000))]
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ChatMessageModel {
    pub id: Uuid,
    pub chat_id: Uuid,
    pub user_id: Uuid,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Chat {
    pub id: Uuid,
    pub name: Option<String>,
    pub chat_type_id: Uuid,
    pub created_at: DateTime<Utc>,
}

