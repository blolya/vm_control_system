use crate::repositories::user_repository::User;
use chrono::{DateTime, NaiveDateTime};
use sqlx::{Error, PgPool};
use uuid::Uuid;

pub struct SshKey {
    pub id: Uuid,
    pub name: String,
    pub key: String,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
}
impl SshKey {
    pub fn id(&self) -> &Uuid {
        &self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn key(&self) -> &str {
        &self.key
    }
    pub fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }
}
#[derive(Clone)]
pub struct SshKeyRepository {
    pool: PgPool,
}
impl SshKeyRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn list(&self) -> Result<Vec<SshKey>, Error> {
        let ssh_keys = sqlx::query_as!(SshKey, "SELECT * FROM ssh_keys;")
            .fetch_all(&self.pool)
            .await?;
        Ok(ssh_keys)
    }
    pub async fn list_for_user(&self, user_id: Uuid) -> Result<Vec<SshKey>, Error> {
        let ssh_keys = sqlx::query_as!(SshKey, "SELECT * FROM ssh_keys WHERE user_id=$1;", user_id)
            .fetch_all(&self.pool)
            .await?;
        Ok(ssh_keys)
    }
    pub async fn create(&self, user_id: Uuid, name: String, key: String) -> Result<SshKey, Error> {
        let ssh_key = sqlx::query_as!(
            SshKey,
            "INSERT INTO ssh_keys(user_id, name, key) VALUES($1, $2, $3) RETURNING *;",
            user_id,
            name,
            key
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(ssh_key)
    }
    pub async fn delete_for_user(&self, user_id: Uuid, id: Uuid) -> Result<SshKey, Error> {
        let ssh_key = sqlx::query_as!(
            SshKey,
            "DELETE FROM ssh_keys WHERE user_id = $1 AND id = $2 RETURNING *;",
            user_id,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(ssh_key)
    }
    pub async fn find_for_user_by_id(&self, user_id: Uuid, id: Uuid) -> Result<SshKey, Error> {
        let ssh_key = sqlx::query_as!(
            SshKey,
            "SELECT * FROM ssh_keys WHERE user_id = $1 AND id = $2;",
            user_id,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(ssh_key)
    }
}
