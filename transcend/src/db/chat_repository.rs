use sqlx::PgPool;
use uuid::Uuid;

use crate::models::chat::{
    ChatCreateModel,
    ChatTypeItemModel,
    ChatMessageModel,
};

pub struct ChatRepository {
    pool: PgPool,
}

impl ChatRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_chat(
        &self,
        model: &ChatCreateModel,
        user_id: Uuid,
    ) -> Result<Uuid, sqlx::Error> {
        // 1️⃣ Create chat
        let chat_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO chats (name, chat_type_id)
            VALUES ($1, $2)
            RETURNING id
            "#
        )
        .bind(&model.name)
        .bind(model.chat_type_id)
        .fetch_one(&self.pool)
        .await?;

        // 2️⃣ Add creator as admin
        sqlx::query(
            r#"
            INSERT INTO chat_users (chat_id, user_id, is_admin)
            VALUES ($1, $2, TRUE)
            "#
        )
        .bind(chat_id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(chat_id)
    }

    pub async fn get_chat_types(
        &self,
    ) -> Result<Vec<ChatTypeItemModel>, sqlx::Error> {
        sqlx::query_as::<_, ChatTypeItemModel>(
            r#"
            SELECT id, type_name
            FROM chat_types
            "#
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn send_message(
        &self,
        chat_id: Uuid,
        user_id: Uuid,
        message: &str,
    ) -> Result<ChatMessageModel, sqlx::Error> {
        // 1️⃣ Check membership
        let is_member: bool = sqlx::query_scalar(
            r#"
            SELECT EXISTS (
                SELECT 1
                FROM chat_users
                WHERE chat_id = $1 AND user_id = $2
            )
            "#
        )
        .bind(chat_id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        if !is_member {
            return Err(sqlx::Error::RowNotFound);
        }

        // 2️⃣ Insert message
        let message = sqlx::query_as::<_, ChatMessageModel>(
            r#"
            INSERT INTO chat_messages (chat_id, user_id, message)
            VALUES ($1, $2, $3)
            RETURNING id, chat_id, user_id, message
            "#
        )
        .bind(chat_id)
        .bind(user_id)
        .bind(message)
        .fetch_one(&self.pool)
        .await?;

        Ok(message)
    }
}
