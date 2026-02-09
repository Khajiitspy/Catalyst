use sqlx::PgPool;

use crate::models::chat::{
    ChatCreateModel,
    ChatTypeItemModel,
    ChatItemModel,
    ChatMessageModel,
    ChatEditModel,
    UserShortModel,
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
        creator_id: i64,
    ) -> Result<i64, sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        // 1️⃣ Create chat
        let chat_id: i64 = sqlx::query_scalar(
            r#"
            INSERT INTO chats (name, chat_type_id)
            VALUES ($1, $2)
            RETURNING id
            "#
        )
        .bind(&model.name)
        .bind(model.chat_type_id)
        .fetch_one(&mut *tx)
        .await?;

        // 2️⃣ Add creator as admin
        sqlx::query(
            r#"
            INSERT INTO chat_users (chat_id, user_id, is_admin)
            VALUES ($1, $2, TRUE)
            "#
        )
        .bind(chat_id)
        .bind(creator_id)
        .execute(&mut *tx)
        .await?;

        // 3️⃣ Add other users
        for user_id in &model.user_ids {
            if *user_id == creator_id {
                continue;
            }

            sqlx::query(
                r#"
                INSERT INTO chat_users (chat_id, user_id, is_admin)
                VALUES ($1, $2, FALSE)
                ON CONFLICT DO NOTHING
                "#
            )
            .bind(chat_id)
            .bind(user_id)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
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

    pub async fn get_user_chats(
        &self,
        user_id: i64,
    ) -> Result<Vec<ChatItemModel>, sqlx::Error> {
        sqlx::query_as::<_, ChatItemModel>(
            r#"
            SELECT
                c.id,
                c.name,
                c.chat_type_id,
                ct.type_name AS chat_type_name
            FROM chats c
            INNER JOIN chat_users cu ON cu.chat_id = c.id
            INNER JOIN chat_types ct ON ct.id = c.chat_type_id
            WHERE cu.user_id = $1
            ORDER BY c.created_at DESC
            "#
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_chat_messages(
        &self,
        chat_id: i64,
        user_id: i64,
    ) -> Result<Vec<ChatMessageModel>, sqlx::Error> {
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

        // 2️⃣ Fetch messages
        sqlx::query_as::<_, ChatMessageModel>(
            r#"
            SELECT
                id,
                chat_id,
                user_id,
                message,
                created_at
            FROM chat_messages
            WHERE chat_id = $1
            ORDER BY created_at ASC
            "#
        )
        .bind(chat_id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn send_message(
        &self,
        chat_id: i64,
        user_id: i64,
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

    pub async fn is_admin(
        &self,
        chat_id: i64,
        user_id: i64,
    ) -> Result<bool, sqlx::Error> {
        sqlx::query_scalar(
            r#"
            SELECT EXISTS (
                SELECT 1
                FROM chat_users
                WHERE chat_id = $1 AND user_id = $2 AND is_admin = TRUE
            )
            "#
        )
        .bind(chat_id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn edit_chat(
        &self,
        model: &ChatEditModel,
        current_user_id: i64,
    ) -> Result<(), sqlx::Error> {
        let is_admin = self.is_admin(model.id, current_user_id).await?;
        if !is_admin {
            return Err(sqlx::Error::RowNotFound);
        }

        if let Some(name) = &model.name {
            sqlx::query("UPDATE chats SET name = $1 WHERE id = $2")
                .bind(name)
                .bind(model.id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(add_ids) = &model.add_user_ids {
            for user_id in add_ids {
                sqlx::query(
                    r#"
                    INSERT INTO chat_users (chat_id, user_id, is_admin)
                    VALUES ($1, $2, FALSE)
                    ON CONFLICT DO NOTHING
                    "#
                )
                .bind(model.id)
                .bind(user_id)
                .execute(&self.pool)
                .await?;
            }
        }

        if let Some(remove_ids) = &model.remove_user_ids {
            sqlx::query(
                r#"
                DELETE FROM chat_users
                WHERE chat_id = $1 AND user_id = ANY($2)
                AND is_admin = FALSE
                "#
            )
            .bind(model.id)
            .bind(remove_ids)
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }

    pub async fn search_users(
        &self,
        query: Option<String>,
        chat_id: Option<i64>,
    ) -> Result<Vec<UserShortModel>, sqlx::Error> {
        let pattern = query.map(|q| format!("%{}%", q.to_lowercase()));

        sqlx::query_as(
            r#"
            SELECT id, first_name || ' ' || last_name AS name
            FROM users
            WHERE
                ($1::TEXT IS NULL OR LOWER(first_name || ' ' || last_name) LIKE $1)
            AND
                ($2::BIGINT IS NULL OR id IN (
                    SELECT user_id FROM chat_users WHERE chat_id = $2
                ))
            "#
        )
        .bind(pattern)
        .bind(chat_id)
        .fetch_all(&self.pool)
        .await
    }
}
