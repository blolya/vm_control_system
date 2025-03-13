use axum::{
    extract::{Path, State},
    routing, Router,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::{
    repositories::ssh_key_repository,
    utils::{
        auth::claims::Claims, jsend::JSend, result::Result,
        validation::validated_json::ValidatedJson,
    },
    AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new().nest(
        "/ssh_keys",
        Router::new()
            .route("/", routing::get(list).post(create))
            .route("/{id}", routing::get(get).delete(delete)),
    )
}

#[derive(Deserialize, Validate)]
struct CreateSshKey {
    name: String,
    key: String,
}

#[derive(Serialize)]
struct SshKey {
    id: Uuid,
    name: String,
    created_at: NaiveDateTime,
}

impl From<ssh_key_repository::SshKey> for SshKey {
    fn from(
        ssh_key_repository::SshKey {
            id,
            name,
            created_at,
            ..
        }: ssh_key_repository::SshKey,
    ) -> Self {
        SshKey {
            id,
            name,
            created_at,
        }
    }
}

async fn list(
    claims: Claims,
    State(AppState {
        ssh_key_repository, ..
    }): State<AppState>,
) -> Result<JSend<Vec<SshKey>>> {
    let ssh_keys = ssh_key_repository
        .list_for_user(claims.data().user_id)
        .await?;
    let ssh_keys = ssh_keys
        .into_iter()
        .map(Into::into)
        .collect::<Vec<SshKey>>();

    Ok(JSend::success(Some(ssh_keys)))
}

async fn create(
    claims: Claims,
    State(AppState {
        ssh_key_repository, ..
    }): State<AppState>,
    ValidatedJson(CreateSshKey { name, key }): ValidatedJson<CreateSshKey>,
) -> Result<JSend<SshKey>> {
    let ssh_key = ssh_key_repository
        .create(claims.data().user_id, name, key)
        .await?;

    Ok(JSend::success(Some(ssh_key.into())))
}
async fn get(
    claims: Claims,
    State(AppState {
        ssh_key_repository, ..
    }): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<JSend<SshKey>> {
    let ssh_key = ssh_key_repository
        .find_for_user_by_id(claims.data().user_id, id)
        .await?;

    Ok(JSend::success(Some(ssh_key.into())))
}
async fn delete(
    claims: Claims,
    State(AppState {
        ssh_key_repository, ..
    }): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<JSend<SshKey>> {
    let ssh_key = ssh_key_repository
        .delete_for_user(claims.data().user_id, id)
        .await?;

    Ok(JSend::success(Some(ssh_key.into())))
}
