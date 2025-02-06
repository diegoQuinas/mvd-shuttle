use sqlx::PgPool;

use crate::models::space;

pub struct CreateSpaceRepository;

impl CreateSpaceRepository {
    pub async fn create(pool: &PgPool, name: &str) -> Result<space::Space, sqlx::Error> {
        let space = sqlx::query_as::<_, space::Space>(
            r#"
        INSERT INTO space (name, created_at, updated_at)
        VALUES ($1, $2, $3)
        RETURNING id, name, created_at, updated_at
        "#,
        )
        .bind(name)
        .bind(chrono::Local::now().naive_local())
        .bind(chrono::Local::now().naive_local())
        .fetch_one(pool)
        .await?;

        Ok(space)
    }

    pub async fn find_by_name(pool: &PgPool, name: &str) -> Option<space::Space> {
        let query = sqlx::query_as(
            r#"
        SELECT id, name, created_at, updated_at
        FROM space
        WHERE name = $1
        "#,
        )
        .bind(name) // Bind del parámetro name
        .fetch_optional(pool) // Ejecuta la consulta y obtiene un resultado opcional
        .await
        .unwrap();
        match query {
            Some(space) => Some(space),
            None => None,
        }
    }
}
