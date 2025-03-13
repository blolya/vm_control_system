use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::repositories::user_repository::User;

#[derive(Serialize)]
pub struct OsImage {
    pub id: Uuid,
    pub name: String,
    pub path: String,
    pub created_at: NaiveDateTime,
}

#[derive(Clone)]
pub struct OsImageRepository {
    pool: PgPool,
}
impl OsImageRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    pub async fn find_by_id(&self, os_image_id: Uuid) -> Result<OsImage, Error> {
        let user = sqlx::query_as!(
            OsImage,
            "SELECT * FROM os_images WHERE id = $1;",
            os_image_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }
    pub async fn list(&self) -> Result<Vec<OsImage>, Error> {
        let os_images = sqlx::query_as!(OsImage, "SELECT * FROM os_images;")
            .fetch_all(&self.pool)
            .await?;

        Ok(os_images)
    }
    pub async fn create(&self, name: String, path: String) -> Result<OsImage, Error> {
        let user = sqlx::query_as!(
            OsImage,
            "INSERT INTO os_images(name, path) VALUES($1, $2) RETURNING *;",
            name,
            path
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }
}
