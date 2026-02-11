use crate::db::chat_repository::ChatRepository;
use sqlx::PgPool;
use crate::{
    models::chat::{
        ChatCreateModel,
        ChatTypeItemModel,
        ChatItemModel,
        ChatMessageModel,
        ChatEditModel,
        UserShortModel,
        UserSearchModel,
    },

    utils::{errors::ApiError},
};

pub struct ChatService {
    repo: ChatRepository,
}

impl ChatService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repo: ChatRepository::new(pool),
        }
    }

    pub async fn create_chat(
        &self,
        model: ChatCreateModel,
        user_id: i64,
    ) -> Result<i64, ApiError> {
        self.repo.create_chat(&model, user_id).await.map_err(ApiError::from)
    }

    pub async fn get_user_chats(
        &self,
        user_id: i64,
    ) -> Result<Vec<ChatItemModel>, ApiError> {
        self.repo.get_user_chats(user_id).await.map_err(ApiError::from)
    }

    pub async fn get_chat_messages(
        &self,
        chat_id: i64,
        user_id: i64,
    ) -> Result<Vec<ChatMessageModel>, ApiError> {
        self.repo.get_chat_messages(chat_id, user_id).await.map_err(ApiError::from)
    }

    pub async fn get_chat_types(
        &self,
    ) -> Result<Vec<ChatTypeItemModel>, ApiError> {
        self.repo.get_chat_types().await.map_err(ApiError::from)
    }

    pub async fn send_message(
        &self,
        chat_id: i64,
        user_id: i64,
        message: String,
    ) -> Result<ChatMessageModel, ApiError> {
        self.repo.send_message(chat_id, user_id, &message).await.map_err(ApiError::from)
    }

    pub async fn edit_chat(
        &self,
        model: ChatEditModel,
        user_id: i64,
    ) -> Result<(), ApiError> {
        self.repo.edit_chat(&model, user_id).await?;
        Ok(())
    }

    pub async fn am_i_admin(
        &self,
        chat_id: i64,
        user_id: i64,
    ) -> Result<bool, ApiError> {
        Ok(self.repo.is_admin(chat_id, user_id).await?)
    }

    pub async fn search_users(
        &self,
        model: UserSearchModel,
    ) -> Result<Vec<UserShortModel>, ApiError> {
        Ok(self
            .repo
            .search_users(model.query, model.chat_id)
            .await?)
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        eprintln!("SQLx error: {err:?}");
        ApiError::InternalServerError
    }
}
