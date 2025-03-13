use std::fmt::Display;

use anyhow::anyhow;
use axum::{extract::State, routing::post, Json, RequestPartsExt, Router};
use jsonwebtoken::{encode, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::utils::jsend::JSend;
use crate::{
    configs::keys::KEYS,
    errors::{auth_error::AuthError, Error},
    repositories::{
        refresh_token_repository::DeleteReturningUsernameResult, user_repository::User,
    },
    utils::{
        auth::claims::{Claims, ClaimsData},
        password::validate_password,
        result::Result,
        validation::validated_json::ValidatedJson,
    },
    AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new().nest(
        "/tokens",
        Router::new()
            .route("/", post(create))
            .route("/refresh", post(refresh_token)),
    )
}

#[derive(Debug, Validate, Deserialize)]
struct Credentials {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: Uuid,
}

async fn create(
    State(AppState {
        user_repository,
        refresh_token_repository,
        ..
    }): State<AppState>,
    ValidatedJson(credentials): ValidatedJson<Credentials>,
) -> Result<JSend<TokenResponse>> {
    let Credentials { username, password } = credentials;
    let User {
        id, password_hash, ..
    } = user_repository
        .get_by_username(&username)
        .await?
        .ok_or(Error::NotFound)?;

    let is_password_valid =
        validate_password(&password, password_hash).map_err(|e| Error::Anyhow(anyhow!(e)))?;
    if !is_password_valid {
        return Err(AuthError::WrongCredentials)?;
    }

    let claims = Claims::new(username, ClaimsData::new(id));
    let access_token = encode(&Header::default(), &claims, &KEYS.encoding())
        .map_err(|e| Error::Anyhow(anyhow!(e)))?;

    refresh_token_repository.delete_all_for_user(id).await?;
    let refresh_token = refresh_token_repository.create(id).await?;

    Ok(JSend::success(Some(TokenResponse {
        access_token,
        refresh_token,
    })))
}

#[derive(Deserialize)]
struct RefreshToken {
    refresh_token: Uuid,
}
async fn refresh_token(
    State(AppState {
        refresh_token_repository,
        ..
    }): State<AppState>,
    Json(RefreshToken { refresh_token }): Json<RefreshToken>,
) -> Result<JSend<TokenResponse>> {
    let DeleteReturningUsernameResult {
        user_id,
        username,
        token_expires_at,
    } = refresh_token_repository
        .delete_returning_username(refresh_token)
        .await?;

    if token_expires_at > chrono::Utc::now().naive_utc() {
        return Err(AuthError::TokenExpired)?;
    }

    let claims = Claims::new(username, ClaimsData::new(user_id));
    let access_token = encode(&Header::default(), &claims, &KEYS.encoding())
        .map_err(|e| Error::Anyhow(anyhow!(e)))?;

    let refresh_token = refresh_token_repository.create(user_id).await?;

    Ok(JSend::success(Some(TokenResponse {
        access_token,
        refresh_token,
    })))
}
