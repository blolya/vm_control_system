use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{error::Error, PgPool};
use uuid::Uuid;
use virt::domain;

use crate::repositories::{ssh_key_repository::SshKey, user_repository::User};

#[derive(Debug, Serialize)]
pub struct Domain {
    id: Uuid,
    user_id: Uuid,
    os_image_id: Uuid,
    name: String,
    vcpu_count: i16,
    memory_size: i16,
    disk_size: i16,
    created_at: NaiveDateTime,
}
impl Domain {
    pub fn id(&self) -> &Uuid {
        &self.id
    }
    pub fn vcpu_count(&self) -> i16 {
        self.vcpu_count
    }
    pub fn memory_size(&self) -> i16 {
        self.memory_size
    }
    pub fn disk_size(&self) -> i16 {
        self.disk_size
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Clone)]
pub struct DomainRepository {
    pool: PgPool,
}
impl DomainRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn list(&self) -> Result<Vec<Domain>, Error> {
        let domains = sqlx::query_as!(Domain, "SELECT * FROM domains;")
            .fetch_all(&self.pool)
            .await?;
        Ok(domains)
    }

    pub async fn create(
        &self,
        id: Uuid,
        user_id: Uuid,
        os_image_id: Uuid,
        name: String,
        vcpu_count: i16,
        memory_size: i16,
        disk_size: i16,
    ) -> Result<Domain, Error> {
        let domain = sqlx::query_as!(
            Domain,
            "INSERT INTO domains(id, user_id, os_image_id, name, vcpu_count, memory_size, disk_size) VALUES($1, $2, $3, $4, $5, $6, $7) RETURNING *;",
            id,
            user_id,
            os_image_id,
            name,
            vcpu_count,
            memory_size,
            disk_size
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(domain)
    }

    pub async fn find_by_id(&self, domain_id: Uuid) -> Result<Domain, Error> {
        let domain = sqlx::query_as!(Domain, "SELECT * FROM domains WHERE id = $1;", domain_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(domain)
    }
    pub async fn delete(&self, domain_id: Uuid) -> Result<Domain, Error> {
        let domain = sqlx::query_as!(
            Domain,
            "DELETE FROM domains WHERE id = $1 RETURNING *;",
            domain_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(domain)
    }
}
