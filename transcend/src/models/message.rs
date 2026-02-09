use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use sqlx::Type;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Message {
    pub id: i64,
    pub chat_id: i64,
    pub sender_id: i64,
    pub content: String,
    pub message_type: MessageType,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "message_type", rename_all = "lowercase")]
pub enum MessageType {
    Text,
    Image,
    File,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct MessageStatus {
    pub message_id: i64,
    pub user_id: i64,
    pub read_at: Option<DateTime<Utc>>,
}
