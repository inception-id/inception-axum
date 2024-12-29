use crate::db::DbPool;
use crate::middleware::{extract_header_user_id, ApiResponse};
use crate::schema::translation;
use crate::translation::services::Translation;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::routing::{post, put};
use axum::{Json, Router};
use diesel::Insertable;
use serde::Deserialize;

type TranslationResponse = (StatusCode, Json<ApiResponse<Translation>>);

#[derive(Deserialize, Insertable)]
#[diesel(table_name = translation)]
pub(super) struct CreateTranslationPayload {
    ai_system_prompt: String,
    content_language: Option<String>,
    target_language: String,
    content: String,
    completion: String,
    updated_completion: String,
}

async fn create_translation_route(
    State(pool): State<DbPool>,
    headers: HeaderMap,
    Json(payload): Json<CreateTranslationPayload>,
) -> TranslationResponse {
    let user_id = extract_header_user_id(headers).expect("Could not extract user id");
    let translation_creation = Translation::create_translation(&pool, &user_id, &payload);
    match translation_creation {
        Ok(translation) => {
            ApiResponse::new(StatusCode::CREATED, Some(translation), "Created").send()
        }
        Err(err) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send()
        }
    }
}

#[derive(Deserialize)]
pub(super) struct UpdateTranslationPayload {
    updated_completion: String,
}
async fn update_translation_route(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTranslationPayload>,
) -> TranslationResponse {
    match Translation::update_translation(&pool, &id, &payload.updated_completion) {
        Ok(translation) => {
            ApiResponse::new(StatusCode::CREATED, Some(translation), "Updated").send()
        }
        Err(err) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send()
        }
    }
}

pub fn translation_routes() -> Router<DbPool> {
    Router::new()
        .route("/create", post(create_translation_route))
        .route("/update/:id", put(update_translation_route))
}
