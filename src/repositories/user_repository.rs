use chrono::{NaiveDateTime, Utc};
use sqlx::{Error, PgPool};
use uuid::Uuid;

pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Clone)]
pub struct UserRepository {
    pool: PgPool,
}
impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, username: String, password_hash: String) -> Result<User, Error> {
        let user = sqlx::query_as!(
            User,
            "INSERT INTO users(username, password_hash) VALUES($1, $2) RETURNING *;",
            username,
            password_hash
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }
    pub async fn update(
        &self,
        user_id: Uuid,
        username: String,
        password_hash: String,
    ) -> Result<User, Error> {
        let user = sqlx::query_as!(
            User,
            "UPDATE users SET username = $1, password_hash = $2, updated_at = $3 WHERE id = $4 RETURNING *;",
            username,
            password_hash,
            Utc::now().naive_utc(),
            user_id
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }
    pub async fn delete(&self, user_id: Uuid) -> Result<User, Error> {
        let user = sqlx::query_as!(
            User,
            "DELETE FROM users WHERE id = $1 RETURNING *;",
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }
    pub async fn find_by_id(&self, user_id: Uuid) -> Result<User, Error> {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1;", user_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }
    pub async fn get_by_username(&self, username: &str) -> Result<Option<User>, Error> {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1;", username)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }
    pub async fn get_by_id(&self, user_id: Uuid) -> Result<Option<User>, Error> {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1;", user_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }
    pub async fn list(&self) -> Result<Vec<User>, Error> {
        let users = sqlx::query_as!(User, "SELECT * FROM users;")
            .fetch_all(&self.pool)
            .await?;

        Ok(users)
    }
}
