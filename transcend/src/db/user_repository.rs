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
            SELECT id, first_name, last_name, email, password_hash, created_at, image
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
        image: Option<&str>,
    ) -> Result<User, sqlx::Error> {
        let rec = sqlx::query_as::<_, User>(
            "INSERT INTO users (first_name, last_name, email, password_hash, image)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING id, first_name, last_name, email, password_hash, image, created_at"
        )
        .bind(first_name)
        .bind(last_name)
        .bind(email)
        .bind(password_hash)
        .bind(image)
        .fetch_one(&self.pool)
        .await?;

        Ok(rec)
    }
}
