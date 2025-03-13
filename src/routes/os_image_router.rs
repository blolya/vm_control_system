use axum::{
    extract::{Path, State},
    routing, Router,
};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::{
    repositories::os_image_repository::OsImage,
    utils::{jsend::JSend, result::Result, validation::validated_json::ValidatedJson},
    AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new().nest(
        "/os_images",
        Router::new()
            .route("/", routing::get(list).post(create))
            .route("/{id}", routing::get(get)),
    )
}

async fn list(
    State(AppState {
        os_image_repository,
        ..
    }): State<AppState>,
) -> Result<JSend<Vec<OsImage>>> {
    let os_images = os_image_repository.list().await?;

    Ok(JSend::success(Some(os_images)))
}

#[derive(Deserialize, Validate)]
struct CreateOsImage {
    name: String,
    path: String,
}
async fn create(
    State(AppState {
        os_image_repository,
        ..
    }): State<AppState>,
    ValidatedJson(CreateOsImage { name, path }): ValidatedJson<CreateOsImage>,
) -> Result<JSend<OsImage>> {
    let os_image = os_image_repository.create(name, path).await?;

    Ok(JSend::success(Some(os_image)))
}
async fn get(
    State(AppState {
        os_image_repository,
        ..
    }): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<JSend<OsImage>> {
    let os_image = os_image_repository.find_by_id(id).await?;

    Ok(JSend::success(Some(os_image)))
}
