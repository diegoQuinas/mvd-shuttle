use axum::{body::Body, extract::State, http::Request, middleware::Next, response::Response};
use cookie::Cookie;
use jsonwebtoken::{decode, Validation};

use crate::{
    errors::AuthError,
    utils::{AppState, Claims},
};

pub async fn auth_middleware(
    State(state): State<AppState>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, AuthError> {
    // Extrae la cookie de la solicitud
    let header_values = request
        .headers()
        .get("Cookie")
        .ok_or(AuthError::Forbidden)?;
    let cookies = Cookie::parse(header_values.to_str().unwrap()).unwrap();

    let cookie = cookies.to_string();
    let token = cookie.trim_start_matches("accessToken=");

    // Verifica el token
    let token_data = decode::<Claims>(token, &state.jwt_secret.decoding, &Validation::default())
        .map_err(|_| AuthError::Forbidden)?;

    // Verifica el rol del usuario (opcional)
    if token_data.claims.role != "admin" {
        return Err(AuthError::Forbidden); // 403 Forbidden
    }

    // Si todo está bien, continúa con la solicitud
    Ok(next.run(request).await)
}
