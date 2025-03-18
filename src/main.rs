mod configs;
mod errors;
mod repositories;
mod routes;
mod utils;
mod views;

use std::{env, error::Error, fmt::Display, sync::Arc, sync::LazyLock, time::Duration};

use axum::{
    extract::{FromRequestParts, State},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, RequestPartsExt, Router,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::{PgPool, PgPoolOptions};
use tracing::error;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use virt::{connect::Connect, error::clear_error_callback};

use crate::errors::handle_404;
use crate::{
    repositories::{
        domain_repository::DomainRepository, os_image_repository::OsImageRepository,
        refresh_token_repository::RefreshTokenRepository, ssh_key_repository::SshKeyRepository,
        user_repository::UserRepository,
    },
    routes::{domain_router, os_image_router, ssh_key_router, token_router, user_router},
    utils::app_state::AppState,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_connection_str = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    clear_error_callback();

    let hypervisor_uri = env::var("HYPERVISOR_URI").ok();
    println!(
        "Attempting to connect to hypervisor: '{:?}'",
        hypervisor_uri
    );

    let hypervisor_connection = match Connect::open(hypervisor_uri.as_deref()) {
        Ok(c) => c,
        Err(e) => panic!("No connection to hypervisor: {}", e),
    };

    let app = Router::new()
        .nest(
            "/api",
            Router::new()
                .merge(user_router::routes())
                .merge(token_router::routes())
                .merge(domain_router::routes())
                .merge(os_image_router::routes())
                .merge(ssh_key_router::routes()),
        )
        .with_state(AppState::new(
            UserRepository::new(pool.clone()),
            RefreshTokenRepository::new(pool.clone()),
            DomainRepository::new(pool.clone()),
            OsImageRepository::new(pool.clone()),
            SshKeyRepository::new(pool.clone()),
            hypervisor_connection,
        ))
        .fallback(handle_404);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
