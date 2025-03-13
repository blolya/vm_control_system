use chrono::NaiveDateTime;
use sqlx::{Error, PgPool};
use uuid::Uuid;

struct RefreshToken {
    token: Uuid,
    user_id: Uuid,
    created_at: NaiveDateTime,
    expires_at: NaiveDateTime,
}
pub struct DeleteReturningUsernameResult {
    pub user_id: Uuid,
    pub username: String,
    pub token_expires_at: NaiveDateTime,
}

#[derive(Clone)]
pub struct RefreshTokenRepository {
    pool: PgPool,
}
impl RefreshTokenRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    pub async fn create(&self, user_id: Uuid) -> Result<Uuid, Error> {
        let refresh_token = sqlx::query_scalar!(
            "INSERT INTO refresh_tokens(user_id) VALUES($1) RETURNING token;",
            user_id,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(refresh_token)
    }
    pub async fn delete_all_for_user(&self, user_id: Uuid) -> Result<(), Error> {
        sqlx::query!("DELETE FROM refresh_tokens WHERE user_id = $1", user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
    pub async fn delete_returning_username(
        &self,
        refresh_token: Uuid,
    ) -> Result<DeleteReturningUsernameResult, Error> {
        let result = sqlx::query_as!(
            DeleteReturningUsernameResult,
            "DELETE FROM refresh_tokens USING users WHERE refresh_tokens.user_id = users.id AND token = $1 RETURNING expires_at as token_expires_at, username, users.id as user_id",
            refresh_token,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }
}
