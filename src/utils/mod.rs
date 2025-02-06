use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use std::sync::Arc;

// implement a method to create a response type containing the JWT
impl AuthBody {
    pub fn new(user: User) -> Self {
        Self {
            status: "success".to_string(),
            user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,  // Subject (usuario)
    pub exp: u64,     // Expiración
    pub role: String, // Rol del usuario
}

// the response that we pass back to HTTP client once successfully authorised
#[derive(Debug, Serialize)]
pub struct AuthBody {
    pub status: String,
    pub user: User,
}

// the request type - "client_id" is analogous to a username, client_secret can also be interpreted as a password
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthRequestPayload {
    pub email: String,
    pub password: String,
}

#[derive(Debug, FromRow, Clone, Deserialize, Serialize)]
pub struct User {
    pub name: String,
    pub rolename: String,
    pub email: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::PgPool,
    pub jwt_secret: Keys,
}

#[derive(Clone)]
pub struct Keys {
    pub encoding: Arc<EncodingKey>,
    pub decoding: Arc<DecodingKey>,
}
