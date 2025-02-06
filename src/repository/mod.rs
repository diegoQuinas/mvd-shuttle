use axum::extract::State;

use crate::utils::{AppState, User};

pub struct UserRepository;

impl UserRepository {
    pub async fn find_by_email(State(state): State<AppState>, email: &str) -> Option<User> {
        match sqlx::query_as::<_, User>(
            r#"
        SELECT id, name, rolename, email, password, created_at, updated_at
        FROM users
        WHERE email = $1
        "#,
        )
        .bind(email) // Bind del parámetro email
        .fetch_optional(&state.pool) // Ejecuta la consulta y obtiene un resultado opcional
        .await
        .unwrap()
        {
            Some(user) => Some(user),
            None => None,
        }
    }
    pub async fn save_user(State(state): State<AppState>, user: User) {
        sqlx::query(
            r#"
        INSERT INTO users (name, rolename, email, password, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        )
        .bind(&user.name)
        .bind(&user.rolename)
        .bind(&user.email)
        .bind(&user.password)
        .bind(&user.created_at)
        .bind(&user.updated_at)
        .execute(&state.pool)
        .await
        .unwrap();
    }
}

pub mod space;
