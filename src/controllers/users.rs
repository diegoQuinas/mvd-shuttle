use std::time::SystemTime;

use axum::{extract::State, http::HeaderMap, Json};
use cookie::{Cookie, SameSite};
use jsonwebtoken::{encode, Header};
use serde::Deserialize;

use crate::{
    errors::AuthError,
    helpers,
    repository::UserRepository,
    utils::{AppState, AuthBody, AuthRequestPayload, Claims, User},
};

#[derive(Deserialize)]
pub struct RegisterRequestPayload {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequestPayload>,
) -> Result<Json<AuthBody>, AuthError> {
    // Check if the user sent the credentials
    let existing_user = UserRepository::find_by_email(State(state.clone()), &payload.email).await;

    if existing_user.is_some() {
        return Err(AuthError::Other("User already exists".to_string()));
    }

    let new_user = User {
        name: payload.name.clone(),
        rolename: "user".to_string(),
        email: payload.email.clone(),
        password: helpers::hash_password::hash_password(payload.password.clone())
            .map_err(|e| AuthError::HashingError(e.to_string()))?,
        created_at: chrono::Local::now().naive_local(),
        updated_at: chrono::Local::now().naive_local(),
    };

    UserRepository::save_user(State(state.clone()), new_user.clone()).await;
    // add 4 minutes to current unix epoch time as expiry date/time
    let exp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 299;

    // Create the authorization token
    let encoding_jwt = &state.jwt_secret.encoding;

    let claims = Claims {
        sub: new_user.email.clone(),
        exp,
        role: "user".to_string(),
    };
    let token =
        encode(&Header::default(), &claims, encoding_jwt).map_err(|_| AuthError::TokenCreation)?;

    set_cookie("accessToken", token);

    // Send the authorized token
    Ok(Json(AuthBody::new(new_user)))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<AuthRequestPayload>,
) -> Result<Json<AuthBody>, AuthError> {
    // Check if the user sent the credentials
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }
    let user = match UserRepository::find_by_email(State(state.clone()), &payload.email).await {
        Some(user) => user,
        None => return Err(AuthError::WrongCredentials),
    };

    match helpers::hash_password::verify_password(user.clone().password, payload.password) {
        Ok(_) => {}
        Err(_) => return Err(AuthError::WrongCredentials),
    }

    // add 4 minutes to current unix epoch time as expiry date/time
    let exp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 299;
    // Create the authorization token
    let encoding_jwt = &state.jwt_secret.encoding;
    let claims = Claims {
        sub: user.email.clone(),
        exp,
        role: user.rolename.clone(),
    };
    let token =
        encode(&Header::default(), &claims, encoding_jwt).map_err(|_| AuthError::TokenCreation)?;

    set_cookie("accessToken", token);

    // Send the authorized token
    Ok(Json(AuthBody::new(user)))
}

pub fn set_cookie(type_of_cookie: &str, token: String) {
    // Construye la cookie
    let cookie = Cookie::build((type_of_cookie, token))
        .http_only(true)
        .secure(true) // Solo en HTTPS
        .same_site(SameSite::Strict)
        .path("/")
        .build();

    // Devuelve la cookie en la respuesta
    let mut headers = HeaderMap::new();
    headers.insert("Set-Cookie", cookie.to_string().parse().unwrap());
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ApiResponse<T> {
    pub status: String,
    pub data: T,
}

impl<T> ApiResponse<T> {
    pub fn new(data: T) -> Self {
        Self {
            status: "success".to_string(),
            data,
        }
    }
}
