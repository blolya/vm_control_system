pub mod auth_error;

use std::{
    borrow::{Borrow, Cow},
    collections::HashMap,
    fmt::Debug,
};

use anyhow::anyhow;
use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json;
use tracing::error;

use crate::{
    errors::{self, auth_error::AuthError},
    utils::{jsend::JSend, validation},
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Unauthorized(#[from] AuthError),

    #[error("resource not found")]
    NotFound,

    #[error("an error occurred within the database")]
    Sqlx(#[from] sqlx::Error),

    #[error("an internal server error occurred")]
    Anyhow(#[from] anyhow::Error),

    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error("incorrect input format")]
    AxumJsonRejection(#[from] JsonRejection),

    #[error("hypervisor error")]
    Hypervisor(#[from] virt::error::Error),
}
impl Error {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::Unauthorized(err) => err.status_code(),
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::ValidationError(_) | Self::AxumJsonRejection(_) => StatusCode::BAD_REQUEST,
            Self::Sqlx(_) | Self::Anyhow(_) | Self::Hypervisor(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Self::Sqlx(ref e) => error!("database error: {:?}", e),
            Self::Anyhow(ref e) => error!("internal server error: {:?}", e),
            Self::Hypervisor(ref e) => error!("hypervisor error: {:?}", e),
            Self::ValidationError(ref e) => {
                error!("validation error: {:?}", e);
                let errors = validation::error::format_validation_errors(&e);
                let body = serde_json::to_string(&errors)
                    .map_err(|e| Self::Anyhow(anyhow!(e)))
                    .unwrap();
                return (self.status_code(), JSend::fail(body)).into_response();
            }
            _ => (),
        };

        (self.status_code(), JSend::fail(self.to_string())).into_response()
    }
}
pub async fn handle_404() -> impl IntoResponse {
    Error::NotFound
}
