use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ChatMessage {
    pub id: Uuid,
    pub chat_id: Uuid,
    pub user_id: Uuid,

    pub message: String,
    pub file_url: Option<String>,

    pub reply_to_message_id: Option<Uuid>,
    pub is_edited: bool,

    pub created_at: DateTime<Utc>,
}
