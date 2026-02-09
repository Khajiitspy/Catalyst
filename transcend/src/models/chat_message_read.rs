use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ChatMessageRead {
    pub message_id: i64,
    pub user_id: i64,
    pub read_at: DateTime<Utc>,
}
