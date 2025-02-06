use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{prelude::FromRow, PgPool, postgres::PgHasArrayType};
use uuid::Uuid;

use crate::utils::AppState;

use super::users::ApiResponse;

pub async fn find_all(State(state): State<AppState>) -> Result<Json<FindAllResponse>, UserError> {
    let members = Member::find_all(&state.pool)
        .await
        .map_err(|e| UserError::InternalServerError(e.to_string()))?;
    Ok(Json(FindAllResponse {
        status: "success".to_string(),
        data: members,
    }))
}

pub async fn find_one(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> Result<Json<FindOneResponse>, UserError> {
    let member = Member::find_by_id(&state.pool, uuid).await;
    match member {
        None => return Err(UserError::NotFound),
        Some(member) => Ok(Json(FindOneResponse {
            status: "success".to_string(),
            data: member,
        })),
    }
}

#[derive(Deserialize)]
pub struct QuerySearch {
    pub name: Option<String>,
    pub lastname: Option<String>,
}
pub async fn get_match_by_name(
    State(state): State<AppState>,
    Query(query): Query<QuerySearch>,
) -> Result<Json<ApiResponse<Vec<MemberSmall>>>, UserError> {
    let member = Member::find_match_by_name(&state.pool, query.name, query.lastname).await?;
    Ok(Json(ApiResponse::new(member)))
}

pub async fn create(
    State(state): State<AppState>,
    Json(member): Json<Member>,
) -> Result<Json<CreateResponse>, UserError> {
    let member = Member::create(&state.pool, member)
        .await
        .map_err(|e| UserError::InternalServerError(e.to_string()))?;
    Ok(Json(CreateResponse {
        status: "success".to_string(),
        data: member,
    }))
}
#[derive(Deserialize)]
pub struct UpdateMemberBody {
    name: Option<String>,
    lastname: Option<String>,
    ci: Option<String>,
    birth_date: Option<NaiveDate>,
    phone: Option<String>,
    tutor_name: Option<String>,
    tutor_lastname: Option<String>,
    tutor_phone: Option<String>,
    observation: Option<String>,
    medical_society_id: Option<Uuid>,
    address: Option<String>,
}
impl Member {
    pub fn update_my_member(&mut self, new_data: UpdateMemberBody) -> () {
        if let Some(name) = new_data.name {
            self.name = name;
        }
        if let Some(lastname) = new_data.lastname {
            self.lastname = lastname;
        }
        if let Some(ci) = new_data.ci {
            self.ci = ci;
        }
        if let Some(birth_date) = new_data.birth_date {
            self.birth_date = birth_date;
        }
        if let Some(phone) = new_data.phone {
            self.phone = phone;
        }
        if let Some(tutor_name) = new_data.tutor_name {
            self.tutor_name = Some(tutor_name);
        }
        if let Some(tutor_lastname) = new_data.tutor_lastname {
            self.tutor_lastname = Some(tutor_lastname);
        }
        if let Some(tutor_phone) = new_data.tutor_phone {
            self.tutor_phone = Some(tutor_phone);
        }
        if let Some(observation) = new_data.observation {
            self.observation = Some(observation)
        }
        if let Some(medical_society_id) = new_data.medical_society_id {
            self.medical_society_id = medical_society_id;
        }
        if let Some(address) = new_data.address {
            self.address = address;
        }
    }
}
pub async fn update(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
    Json(update_member_body): Json<UpdateMemberBody>,
) -> Result<Json<UpdateResponse>, UserError> {
    let mut member = Member::find_by_id(&state.pool, uuid.clone())
        .await
        .ok_or(UserError::NotFound)?;

    member.update_my_member(update_member_body);

    let updated_member = Member::update(&state.pool, member)
        .await
        .map_err(|e| UserError::InternalServerError(e.to_string()))?;
    Ok(Json(UpdateResponse {
        status: "success".to_string(),
        data: updated_member,
    }))
}

pub async fn delete(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> Result<Json<DeleteResponse>, UserError> {
    Member::delete(&state.pool, uuid)
        .await
        .map_err(|e| UserError::InternalServerError(e.to_string()))?;
    Ok(Json(DeleteResponse {
        status: "success".to_string(),
    }))
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserError {
    NotFound,
    InternalServerError(String),
}

impl IntoResponse for UserError {
    fn into_response(self) -> Response<Body> {
        let (status, message) = match self {
            UserError::NotFound => (StatusCode::NOT_FOUND, "User not found".to_string()),
            UserError::InternalServerError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e),
        };

        let body = Json(json!({
            "status": "error",
            "error": message,
        }));
        (status, body).into_response()
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FindAllResponse {
    pub status: String,
    pub data: Vec<Member>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FindOneResponse {
    pub status: String,
    pub data: Member,
}
#[derive(Debug, Serialize, Deserialize)]

pub struct CreateResponse {
    pub status: String,
    pub data: Member,
}
#[derive(Debug, Serialize, Deserialize)]

pub struct UpdateResponse {
    pub status: String,
    pub data: Member,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteResponse {
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Member {
    id: Uuid,
    name: String,
    lastname: String,
    ci: String,
    birth_date: NaiveDate,
    phone: String,
    tutor_name: Option<String>,
    tutor_lastname: Option<String>,
    tutor_phone: Option<String>,
    observation: Option<String>,
    medical_society_id: Uuid,
    address: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}


#[derive(Serialize, Deserialize, FromRow)]
pub struct MemberSmall {
    pub id: Uuid,
    pub name: String,
    pub lastname: String,
    pub ci: String,
    pub birth_date: NaiveDate,
}

impl Member {

    pub async fn find_match_by_name(
        pool: &PgPool,
        name: Option<String>,
        lastname: Option<String>,
    ) -> Result<Vec<MemberSmall>, UserError> {
        if name.is_some() && lastname.is_some() {
            let query: Vec<MemberSmall> = sqlx::query_as(
            r#"
            SELECT id, name, lastname, ci, birth_date
            FROM members
            WHERE name ILIKE $1 AND lastname ILIKE $2 
            "#,
        )
        .bind(format!("{}%", name.unwrap()))
        .bind(format!("{}%", lastname.unwrap()))
        .fetch_all(pool)
        .await
        .map_err(|e| UserError::InternalServerError(e.to_string()))?;
            return Ok(query);
        } else if name.is_some() && lastname.is_none() {
            let query = sqlx::query_as(
                r#"
                SELECT id, name, lastname, ci, birth_date
                FROM members
                WHERE name ILIKE $1
                "#,
            )
            .bind(format!("{}%", name.unwrap()))
            .fetch_all(pool)
            .await
            .map_err(|e| UserError::InternalServerError(e.to_string()))?;
            return Ok(query);
        } else if name.is_none() && lastname.is_some() {
            let query = sqlx::query_as(
                r#"
                SELECT id, name, lastname, ci, birth_date
                FROM members
                WHERE lastname ILIKE $1
                "#,
            )
            .bind(format!("{}%", lastname.unwrap()))
            .fetch_all(pool)
            .await
            .map_err(|e| UserError::InternalServerError(e.to_string()))?;
            return Ok(query);
        } else {
            return Err(UserError::InternalServerError(
                "No se ingresaron datos".to_string(),
            ));
        }
    }

    pub async fn find_all(pool: &PgPool) -> Result<Vec<Member>, sqlx::Error> {
        let members = sqlx::query_as::<_, Member>(
            r#"
            SELECT id, name, lastname, ci, birth_date, phone, tutor_name, tutor_lastname, tutor_phone, observation, medical_society_id, address, created_at, updated_at
            FROM members
            "#,
        )
        .fetch_all(pool)
        .await?;
        Ok(members)
    }

    pub struct FullMemberDetail {
        id: Uuid,
        name: String,
        lastname: String,
        ci: String,
        birth_date: NaiveDate,
        phone: String,
        tutor_name: Option<String>,
        tutor_lastname: Option<String>,
        tutor_phone: Option<String>,
        observation: Option<String>,
        medical_society: MedicalSocietyInfo,
        activities: Vec<Activity>,
        address: String,
        created_at: NaiveDateTime,
        updated_at: NaiveDateTime,
        dues: Vec<Due>,
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Option<Member> {
        let mut result: FullMemberDetail;
        let query = sqlx::query_as(
            r#"
            SELECT id, name, lastname, ci, birth_date, phone, tutor_name, tutor_lastname, tutor_phone, observation, medical_society_id, address, created_at, updated_at
            FROM members
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await
        .unwrap();
        match query {
            Some(member) => Some(member),
            None => None,
        }
    }

    pub async fn create(pool: &PgPool, member: Member) -> Result<Member, sqlx::Error> {
        let member = sqlx::query_as::<_, Member>(
            r#"
            INSERT INTO members (name, lastname, ci, birth_date, phone, tutor_name, tutor_lastname, tutor_phone, observation, medical_society_id, address, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            RETURNING id, name, lastname, ci, birth_date, phone, tutor_name, tutor_lastname, tutor_phone, observation, medical_society_id, address, created_at, updated_at
            "#,
        )
        .bind(&member.name)
        .bind(&member.lastname)
        .bind(&member.ci)
        .bind(&member.birth_date)
        .bind(&member.phone)
        .bind(&member.tutor_name)
        .bind(&member.tutor_lastname)
        .bind(&member.tutor_phone)
        .bind(&member.observation)
        .bind(&member.medical_society_id)
        .bind(&member.address)
        .bind(&member.created_at)
        .bind(&member.updated_at)
        .fetch_one(pool)
        .await?;

        Ok(member)
    }

    pub async fn update(pool: &PgPool, member: Member) -> Result<Member, sqlx::Error> {
        let member = sqlx::query_as::<_, Member>(
            r#"
            UPDATE members
            SET name = $1, lastname = $2, ci = $3, birth_date = $4, phone = $5, tutor_name = $6, tutor_lastname = $7, tutor_phone = $8, observation = $9, medical_society_id = $10, address = $11, updated_at = $12
            WHERE id = $13
            RETURNING id, name, lastname, ci, birth_date, phone, tutor_name, tutor_lastname, tutor_phone, observation, medical_society_id, address, created_at, updated_at
            "#,
        )
        .bind(&member.name)
        .bind(&member.lastname)
        .bind(&member.ci)
        .bind(&member.birth_date)
        .bind(&member.phone)
        .bind(&member.tutor_name)
        .bind(&member.tutor_lastname)
        .bind(&member.tutor_phone)
        .bind(&member.observation)
        .bind(&member.medical_society_id)
        .bind(&member.address)
        .bind(chrono::DateTime::from_timestamp(chrono::Local::now().timestamp(), 0))
        .bind(&member.id)
        .fetch_one(pool)
        .await?;

        Ok(member)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            DELETE FROM members
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(pool)
        .await?;

        Ok(())
    }
}
