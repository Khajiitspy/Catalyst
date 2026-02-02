use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ChatUser {
    pub chat_id: Uuid,
    pub user_id: Uuid,
    pub is_admin: bool,
    pub joined_at: DateTime<Utc>,
}
