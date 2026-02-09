use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ChatMessage {
    pub id: i64,
    pub chat_id: i64,
    pub user_id: i64,

    pub message: String,
    pub file_url: Option<String>,

    pub reply_to_message_id: Option<i64>,
    pub is_edited: bool,

    pub created_at: DateTime<Utc>,
}
