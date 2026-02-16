use sqlx::{PgPool, Error};
use crate::models::user::{EditProfileRequest, User};

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

        pub async fn update_user(
        &self,
        user_id: i64,
        edit: EditProfileRequest,
        image: Option<&str>,
    ) -> Result<User, Error> {
        sqlx::query_as::<_, User>(r#"
            UPDATE users
            SET
                first_name = COALESCE($2, first_name),
                last_name  = COALESCE($3, last_name),
                email      = COALESCE($4, email),
                image      = COALESCE($5, image)
            WHERE id = $1
            RETURNING
                id,
                first_name,
                last_name,
                email,
                password_hash,
                image,
                created_at
        "#)
        .bind(user_id)          // $1
        .bind(edit.first_name)  // $2
        .bind(edit.last_name)   // $3
        .bind(edit.email)       // $4
        .bind(image)            // $5 (Option<&str>)
        .fetch_one(&self.pool)
        .await
    }
}
