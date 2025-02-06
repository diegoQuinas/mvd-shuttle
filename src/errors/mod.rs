use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

// implement IntoResponse for AuthError so we can use it as an Axum response type
impl IntoResponse for AuthError {
    fn into_response(self) -> Response<Body> {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (
                StatusCode::UNAUTHORIZED,
                "Credenciales incorrectas".to_string(),
            ),
            AuthError::MissingCredentials => (
                StatusCode::BAD_REQUEST,
                "Faltan las credenciales".to_string(),
            ),
            AuthError::TokenCreation => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error al crear token".to_string(),
            ),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Token inválido".to_string()),
            AuthError::HashingError(e) => {
                let error = format!("Problema al hashear: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, error)
            }
            AuthError::DatabaseError(e) => {
                let error = format!("Error en la base de datos: {}", e.to_string());
                (StatusCode::INTERNAL_SERVER_ERROR, error)
            }
            AuthError::MissingToken => (StatusCode::BAD_REQUEST, "Falta el token".to_string()),
            AuthError::Other(e) => (StatusCode::INTERNAL_SERVER_ERROR, e),
            AuthError::MissingAppState => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Falta el AppState".to_string(),
            ),
            AuthError::Forbidden => (StatusCode::FORBIDDEN, "Acceso no autorizado".to_string()),
            AuthError::UserNotFound(email) => (
                StatusCode::NOT_FOUND,
                format!("Usuario \"{}\" no encontrado", email),
            ),
        };

        let body = Json(json!({
            "status": "error",
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

// error types for auth errors
#[derive(Debug)]
pub enum AuthError {
    MissingAppState,
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
    HashingError(String),
    DatabaseError(sqlx::Error),
    MissingToken,
    Other(String), // Otros errores
    Forbidden,
    UserNotFound(String),
}
