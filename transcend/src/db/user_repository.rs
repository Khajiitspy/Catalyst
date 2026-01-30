use sqlx::PgPool;
use crate::models::user::User;

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_email(
        &self,
        email: &str,
    ) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"
            SELECT id, first_name, last_name, email, password_hash, created_at, image_filename
            FROM users
            WHERE email = $1
            "#
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn create_user(
        &self,
        first_name: &str,
        last_name: &str,
        email: &str,
        password_hash: &str,
        image_filename: Option<&str>,
    ) -> Result<User, sqlx::Error> {
        let rec = sqlx::query_as::<_, User>(
            "INSERT INTO users (first_name, last_name, email, password_hash, image_filename)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING id, first_name, last_name, email, password_hash, image_filename, created_at"
        )
        .bind(first_name)
        .bind(last_name)
        .bind(email)
        .bind(password_hash)
        .bind(image_filename)
        .fetch_one(&self.pool)
        .await?;

        Ok(rec)
    }
}
