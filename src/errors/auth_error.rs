use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("wrong credentials")]
    WrongCredentials,
    #[error("missing credentials")]
    MissingCredentials,
    #[error("invalid token")]
    InvalidToken,
    #[error("token expired")]
    TokenExpired,
}
impl AuthError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AuthError::MissingCredentials => StatusCode::BAD_REQUEST,
            _ => StatusCode::UNAUTHORIZED,
        }
    }
}
impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        (self.status_code(), self.to_string()).into_response()
    }
}
