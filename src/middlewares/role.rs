use axum::{
    extract::{Extension, Request, State},
    middleware::Next,
    response::Response,
};

use crate::{
    errors::AuthError,
    utils::{Claims, User},
};

pub async fn role_middleware(
    Extension((_claims, user)): Extension<(Claims, User)>,
    request: Request,
    next: Next,
) -> Result<Response, AuthError> {
    // Verificar el rol del usuario
    if user.rolename != "admin" {
        return Err(AuthError::Forbidden);
    }

    // Permitir el acceso si el rol es válido
    Ok(next.run(request).await)
}
