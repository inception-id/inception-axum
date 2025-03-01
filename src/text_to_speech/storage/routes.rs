use super::services::TextToSpeechStorage;
use crate::db::DbPool;
use crate::languageai_subscriptions::{SubcriptionLimit, SubcriptionStorageLimit};
use crate::middleware::{extract_header_user_id, ApiResponse};
use crate::schema;
use crate::text_to_speech::services::TextToSpeech;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use diesel::AsChangeset;
use serde::Deserialize;

type TtsStorageResponse = (StatusCode, Json<ApiResponse<TextToSpeechStorage>>);

#[derive(Deserialize)]
pub(crate) struct CreateTtsStoragePayload {
    tts_id: i32,
    title: Option<String>,
}
pub(crate) async fn create_tts_storage_route(
    State(pool): State<DbPool>,
    headers: HeaderMap,
    Json(payload): Json<CreateTtsStoragePayload>,
) -> TtsStorageResponse {
    let user_id = extract_header_user_id(headers).expect("Could not extract user id");

    match SubcriptionLimit::check_user_exceed_limit(
        &pool,
        &user_id,
        &SubcriptionLimit::Storage,
        &Some(SubcriptionStorageLimit::TextToSpeech),
    ) {
        true => ApiResponse::new(
            StatusCode::PAYMENT_REQUIRED,
            None,
            &StatusCode::PAYMENT_REQUIRED.to_string(),
        )
        .send(),
        false => match TextToSpeech::find_by_id(&pool, &payload.tts_id) {
            Ok(tts) => match TextToSpeechStorage::create_tts_storage(&pool, &tts, &payload.title) {
                Ok(tts_storage) => {
                    ApiResponse::new(StatusCode::CREATED, Some(tts_storage), "Created").send()
                }
                Err(storage_error) => ApiResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    None,
                    &storage_error.to_string(),
                )
                .send(),
            },
            Err(err) => {
                ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send()
            }
        },
    }
}

pub(crate) async fn find_tts_storage_route(
    pool: State<DbPool>,
    headers: HeaderMap,
) -> (StatusCode, Json<ApiResponse<Vec<TextToSpeechStorage>>>) {
    let user_id = extract_header_user_id(headers).expect("Could not extract user id");
    let storage_limit = SubcriptionLimit::find_user_subscription_limit_count(
        &pool,
        &user_id,
        &SubcriptionLimit::Storage,
    );
    match TextToSpeechStorage::find_many_tts_storage(&pool, &user_id, &storage_limit) {
        Ok(tts_storage) => ApiResponse::new(StatusCode::OK, Some(tts_storage), "Success").send(),
        Err(err) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send()
        }
    }
}

pub(crate) async fn delete_tts_storage_route(
    pool: State<DbPool>,
    Path(id): Path<i32>,
) -> TtsStorageResponse {
    match TextToSpeechStorage::delete_tts_storage(&pool, &id) {
        Ok(tts_storage) => ApiResponse::new(StatusCode::OK, Some(tts_storage), "Success").send(),
        Err(err) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send()
        }
    }
}

#[derive(Deserialize, AsChangeset)]
#[diesel(table_name = schema::text_to_speech_storage)]
pub(crate) struct UpdateTtsStoragePayload {
    title: Option<String>,
}

pub(crate) async fn update_tts_storage_route(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTtsStoragePayload>,
) -> TtsStorageResponse {
    match TextToSpeechStorage::update_tts_storage(&pool, &id, &payload.title) {
        Ok(tts_storage) => ApiResponse::new(StatusCode::OK, Some(tts_storage), "Success").send(),
        Err(err) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send()
        }
    }
}
