use sqlx::{PgPool, Error};
use crate::models::chat::ChatTypeItemModel;  // Import the ChatTypeItemModel

pub async fn seed_data(pool: &PgPool) -> Result<(), Error> {
    // Check if the ChatTypes table is empty and insert default data
    let chatTypeRows: Vec<ChatTypeItemModel> = sqlx::query_as("SELECT * FROM chat_types")
        .fetch_all(pool)
        .await?;

    if chatTypeRows.is_empty() {
        // Insert default ChatTypes if the table is empty
        let chat_types = vec![
            "Private",
            "Group",
        ];

        // Insert each chat type into the database
        for chat_type in chat_types {
            sqlx::query("INSERT INTO chat_types (type_name) VALUES ($1)")
                .bind(chat_type)
                .execute(pool)
                .await?;
        }
        println!("ChatTypes table seeded with default values.");
    }

    let roleRows: Vec<ChatTypeItemModel> = sqlx::query_as("SELECT * FROM roles")
        .fetch_all(pool)
        .await?;

    if roleRows.is_empty() {
        let roles = vec![
            "Admin", 
            "User",
        ];

        // Insert each chat type into the database
        for role in roles {
            sqlx::query("INSERT INTO roles (name) VALUES ($1)")
                .bind(role)
                .execute(pool)
                .await?;
        }
        println!("Roles table seeded with default values.");
    }

    Ok(())
}
