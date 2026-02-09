use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ChatUser {
    pub chat_id: i64,
    pub user_id: i64,
    pub is_admin: bool,
    pub joined_at: DateTime<Utc>,
}
