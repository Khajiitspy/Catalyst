use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::Type;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Message {
    pub id: Uuid,
    pub chat_id: Uuid,
    pub sender_id: Uuid,
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
    pub message_id: Uuid,
    pub user_id: Uuid,
    pub read_at: Option<DateTime<Utc>>,
}
