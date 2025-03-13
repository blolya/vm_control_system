use std::fmt::Display;

use axum::{extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, TokenData, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    configs::keys::KEYS, errors::auth_error::AuthError, errors::Error, utils::result::Result,
    AppState,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct ClaimsData {
    pub user_id: Uuid,
}
impl ClaimsData {
    pub fn new(user_id: Uuid) -> Self {
        Self { user_id }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    data: ClaimsData,
    exp: u64,
    iat: u64,
    sub: String,
}
impl Claims {
    pub fn new(sub: String, data: ClaimsData) -> Self {
        let now = Utc::now();
        let iat = now.timestamp() as u64;
        let exp = (now + Duration::minutes(15)).timestamp() as u64;

        Self {
            data,
            exp,
            iat,
            sub,
        }
    }
    pub fn data(&self) -> &ClaimsData {
        &self.data
    }
    pub fn sub(&self) -> &str {
        &self.sub
    }
}
impl FromRequestParts<AppState> for Claims {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &AppState) -> Result<Self> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        let token: TokenData<Claims> =
            decode(bearer.token(), &KEYS.decoding(), &Validation::default()).map_err(
                |e| match e.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::TokenExpired,
                    _ => AuthError::InvalidToken,
                },
            )?;

        Ok(token.claims)
    }
}
