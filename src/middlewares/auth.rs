use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, Validation};

use crate::{
    errors::AuthError,
    repository::UserRepository,
    utils::{AppState, Claims},
};

pub async fn jwt_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AuthError> {
    // Extraer el token del encabezado Authorization
    let token = request
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "))
        .ok_or(AuthError::MissingToken)?;

    // Decodificar el JWT
    let decoding_key = state.jwt_secret.decoding.clone();
    let validation = Validation::default();

    match decode::<Claims>(token, &decoding_key, &validation) {
        Ok(token_data) => {
            // Insertar los claims en la solicitud para su uso posterior,
            let claims = token_data.claims.clone();
            let user =
                UserRepository::find_by_email(axum::extract::State(state.clone()), &claims.sub)
                    .await;
            if let None = user {
                return Err(AuthError::InvalidToken);
            } else {
                request.extensions_mut().insert((user, claims));
                Ok(next.run(request).await)
            }
        }
        Err(_) => Err(AuthError::InvalidToken),
    }
}
