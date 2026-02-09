use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ChatTypeItemModel {
    pub id: i64,
    pub type_name: String,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChatCreateModel {
    #[validate(length(min = 1))]
    pub name: Option<String>,
    pub chat_type_id: i64,
    pub user_ids: Vec<i64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSearchModel {
    pub query: Option<String>,
    pub chat_id: Option<i64>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct UserShortModel {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChatEditModel {
    pub id: i64,

    #[validate(length(min = 1))]
    pub name: Option<String>,

    pub add_user_ids: Option<Vec<i64>>,
    pub remove_user_ids: Option<Vec<i64>>,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SendMessageModel {
    pub chat_id: i64,

    #[validate(length(min = 1, max = 1000))]
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessageModel {
    pub id: i64,
    pub chat_id: i64,
    pub user_id: i64,
    pub message: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ChatItemModel {
    pub id: i64,
    pub name: String,
    pub chat_type_id: i64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Chat {
    pub id: i64,
    pub name: Option<String>,
    pub chat_type_id: i64,
    pub created_at: DateTime<Utc>,
}

