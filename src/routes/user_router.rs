use anyhow::anyhow;
use axum::{
    extract::{Path, State},
    routing, Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::{
    utils::{
        auth::claims::Claims, jsend::JSend, password::hash_password, result::Result,
        validation::validated_json::ValidatedJson,
    },
    AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new().nest(
        "/users",
        Router::new()
            .route("/", routing::get(list).post(create))
            .route(
                "/{id}",
                routing::get(get).delete(delete).put(update).patch(upgrade),
            ),
    )
}

#[derive(Deserialize, Validate)]
struct CreateUser {
    username: String,
    password: String,
}
#[derive(Deserialize)]
struct UpdateUser {
    username: String,
    password: String,
}
#[derive(Deserialize)]
struct UpgradeUser;
#[derive(Serialize)]
struct User {
    id: Uuid,
    username: String,
}
impl From<crate::repositories::user_repository::User> for User {
    fn from(value: crate::repositories::user_repository::User) -> Self {
        Self {
            id: value.id,
            username: value.username,
        }
    }
}

#[derive(Serialize)]
struct Users {
    users: Vec<User>,
}
impl Users {
    fn new(users: Vec<User>) -> Self {
        Self { users }
    }
}

async fn list(
    _claims: Claims,
    State(AppState {
        user_repository, ..
    }): State<AppState>,
) -> Result<JSend<Vec<User>>> {
    let users = user_repository.list().await?;
    let users = users
        .into_iter()
        .map(|user| user.into())
        .collect::<Vec<User>>();

    Ok(JSend::success(Some(users)))
}

async fn create(
    // _claims: Claims,
    State(AppState {
        user_repository, ..
    }): State<AppState>,
    ValidatedJson(CreateUser { username, password }): ValidatedJson<CreateUser>,
) -> Result<JSend<User>> {
    let password_hash = hash_password(&password).map_err(|err| anyhow!(err))?;
    let user = user_repository.create(username, password_hash).await?;

    Ok(JSend::success(Some(user.into())))
}
async fn get(
    _claims: Claims,
    State(AppState {
        user_repository, ..
    }): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<JSend<User>> {
    let user = user_repository.find_by_id(id).await?;

    Ok(JSend::success(Some(user.into())))
}
async fn update(
    _claims: Claims,
    State(AppState {
        user_repository, ..
    }): State<AppState>,
    Path(id): Path<Uuid>,
    Json(UpdateUser { username, password }): Json<UpdateUser>,
) -> Result<JSend<User>> {
    let password_hash = hash_password(&password).map_err(|err| anyhow!(err))?;
    let user = user_repository.update(id, username, password_hash).await?;

    Ok(JSend::success(Some(user.into())))
}
async fn upgrade(
    State(AppState {
        user_repository, ..
    }): State<AppState>,
    Path(_id): Path<Uuid>,
    Json(UpgradeUser): Json<UpgradeUser>,
) -> Result<Json<User>, ()> {
    let user = User {
        id: Uuid::new_v4(),
        username: "John".to_string(),
    };

    Ok(Json(user))
}
async fn delete(
    _claims: Claims,
    State(AppState {
        user_repository, ..
    }): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<JSend<User>> {
    let user = user_repository.delete(id).await?;

    Ok(JSend::success(Some(user.into())))
}
