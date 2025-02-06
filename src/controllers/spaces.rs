use axum::{
    body::{self, Body},
    extract::{Query, State},
    http::{status, Response, StatusCode},
    response::IntoResponse,
    Json,
};
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{repository::space::CreateSpaceRepository, utils::AppState};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseSpace {
    pub id: String,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SpaceError {
    NotFound,
    InternalServerError(String),
}

impl IntoResponse for SpaceError {
    fn into_response(self) -> Response<Body> {
        let (status, message) = match self {
            SpaceError::NotFound => (StatusCode::NOT_FOUND, "Space not found".to_string()),
            SpaceError::InternalServerError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e),
        };

        let body = Json(json!({
            "status": "error",
            "error": message,
        }));
        (status, body).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpaceCreateRequest {
    name: String,
}

#[derive(Deserialize)]
pub struct SpaceQuery {
    name: String,
}

pub async fn create(
    State(state): State<AppState>,
    Json(body): Json<SpaceCreateRequest>,
) -> Result<Json<ResponseSpace>, SpaceError> {
    if CreateSpaceRepository::find_by_name(&state.pool, &body.name)
        .await
        .is_some()
    {
        return Err(SpaceError::InternalServerError(format!(
            "Space \"{}\" already exists",
            &body.name
        )));
    }

    match CreateSpaceRepository::create(&state.pool, &body.name).await {
        Ok(space) => Ok(Json(ResponseSpace {
            id: space.id.to_string(),
            name: space.name,
            created_at: space.created_at,
            updated_at: space.updated_at,
        })),
        Err(e) => Err(SpaceError::InternalServerError(e.to_string())),
    }
}

pub async fn find_by_name(
    State(state): State<AppState>,
    Query(query): Query<SpaceQuery>,
) -> Result<Json<ResponseSpace>, SpaceError> {
    match CreateSpaceRepository::find_by_name(&state.pool, &query.name).await {
        Some(space) => Ok(Json(ResponseSpace {
            id: space.id.to_string(),
            name: space.name,
            created_at: space.created_at,
            updated_at: space.updated_at,
        })),
        None => Err(SpaceError::NotFound),
    }
}
