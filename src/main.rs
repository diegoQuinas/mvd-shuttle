use std::sync::Arc;

use axum::{
    http::{self, HeaderValue, Method},
    middleware,
    routing::{delete, get, patch, post},
    Router,
};
use axum_extra::headers::Origin;
use jsonwebtoken::{DecodingKey, EncodingKey};
use shuttle_runtime::SecretStore;
use sqlx::{self};

use mvd_shuttle::{
    controllers, middlewares,
    utils::{AppState, Keys},
};
use tower_http::cors::{Any, CorsLayer};
#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
    #[shuttle_shared_db::Postgres] db_url: String,
) -> shuttle_axum::ShuttleAxum {
    // Conectar a la base de datos
    let pool = sqlx::PgPool::connect(&db_url)
        .await
        .expect("Failed to connect to Postgres");

    // Ejecutar migraciones
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    // Obtener el secreto JWT desde Secrets.toml
    let jwt_secret = secrets
        .get("JWT_SECRET")
        .expect("JWT_SECRET no está definido en Secrets.toml");

    // Crear el estado de la aplicación
    let state = AppState {
        pool,
        jwt_secret: Keys {
            encoding: Arc::new(EncodingKey::from_secret(jwt_secret.as_bytes())),
            decoding: Arc::new(DecodingKey::from_secret(jwt_secret.as_bytes())),
        },
    };

    let frontend_url = secrets
        .get("FRONTEND_URL")
        .expect("FRONTEND_URL no está definido en Secrets.toml");

    // Configura CORS
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PATCH]) // Métodos permitidos
        .allow_origin(frontend_url.parse::<HeaderValue>().unwrap())
        .allow_headers([http::header::CONTENT_TYPE])
        .allow_credentials(true);

    // Configurar la aplicación
    let app = Router::new()
        .route("/api/v1/user/login", post(controllers::users::login))
        .route("/api/v1/user/register", post(controllers::users::register))
        .route("/api/v1/space/create", post(controllers::spaces::create))
        .route("/api/v1/space/find", get(controllers::spaces::find_by_name))
        .route(
            "/api/v1/medical_societies",
            get(controllers::medical_society::get_medical_societies).layer(
                middleware::from_fn_with_state(
                    state.clone(),
                    middlewares::auth_middleware::auth_middleware,
                ),
            ),
        )
        .route("/api/v1/members/create", post(controllers::members::create))
        .route(
            "/api/v1/members/find",
            get(controllers::members::get_match_by_name),
        )
        //.route("/api/v1/members/:uuid", get(controllers::members::find_one))
        .route("/api/v1/members", get(controllers::members::find_all))
        //.route("/api/v1/members/:uuid", patch(controllers::members::update))
        .route(
            "/api/v1/members/:uuid",
            delete(controllers::members::delete),
        )
        .route(
            "/api/v1/members/find_by_name",
            get(controllers::members::get_match_by_name),
        )
        .layer(cors)
        .with_state(state); // Pasar el estado a los manejadores

    Ok(app.into())
}
