use axum::{
    body::Body,
    extract::State,
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{prelude::FromRow, PgPool};
use uuid::Uuid;

use crate::utils::{AppState, Claims};

pub async fn get_medical_societies(
    State(state): State<AppState>,
) -> Result<Json<MedicalSocietiesResponse>, MedicalSocietiesError> {
    let medical_societies: Vec<MedicalSociety> = MedicalSociety::find_all(&state.pool)
        .await
        .unwrap_or(vec![]);
    let body = Json(MedicalSocietiesResponse {
        status: "success".to_string(),
        data: medical_societies,
    });
    Ok(body)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MedicalSocietiesError {
    NotFound,
    InternalServerError(String),
}
impl IntoResponse for MedicalSocietiesError {
    fn into_response(self) -> Response<Body> {
        let (status, message) = match self {
            MedicalSocietiesError::NotFound => (
                StatusCode::NOT_FOUND,
                "Medical Societies not found".to_string(),
            ),
            MedicalSocietiesError::InternalServerError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e),
        };
        let body = Json(json!({
            "status": "error",
            "error": message,
        }));
        (status, body).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MedicalSocietiesResponse {
    pub status: String,
    pub data: Vec<MedicalSociety>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MedicalSociety {
    pub id: Uuid,
    pub name: String,
    pub emergency_phone: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl MedicalSociety {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<MedicalSociety>, sqlx::Error> {
        let medical_societies = sqlx::query_as::<_, MedicalSociety>(
            r#"
            SELECT id, name, emergency_phone, created_at, updated_at
            FROM medical_society
            "#,
        )
        .fetch_all(pool)
        .await?;
        Ok(medical_societies)
    }
}
