use super::model::ApiKey;
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    middleware::from_fn,
    routing::{delete, get, post},
    Router,
};
use serde::Serialize;

use crate::{
    db::DbPool,
    middleware::{extract_session_user_id, session_middleware, AxumResponse, JsonResponse},
};

async fn create_api_key(headers: HeaderMap, State(pool): State<DbPool>) -> AxumResponse<String> {
    let user_id = match extract_session_user_id::<String>(&headers) {
        Ok(id) => id,
        Err(err) => return JsonResponse::send(403, None, Some(err.to_string())),
    };

    let (user_key, hashed_key) = match ApiKey::create_key() {
        Ok(keys) => keys,
        Err(err) => return JsonResponse::send(500, None, Some(err.to_string())),
    };

    match ApiKey::create(&pool, &user_id, &hashed_key) {
        Ok(_) => JsonResponse::send(201, Some(user_key), None),
        Err(err) => return JsonResponse::send(500, None, Some(err.to_string())),
    }
}

#[derive(Serialize)]
pub struct FindApiKeysResponse {
    id: uuid::Uuid,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
}
async fn find_api_keys(
    headers: HeaderMap,
    State(pool): State<DbPool>,
) -> AxumResponse<Vec<FindApiKeysResponse>> {
    let user_id = match extract_session_user_id::<String>(&headers) {
        Ok(id) => id,
        Err(err) => return JsonResponse::send(403, None, Some(err.to_string())),
    };

    let api_keys: Vec<ApiKey> = match ApiKey::find_many(&pool, &user_id) {
        Ok(keys) => keys,
        Err(err) => return JsonResponse::send(500, None, Some(err.to_string())),
    };
    let res = api_keys
        .into_iter()
        .map(|key| FindApiKeysResponse {
            id: key.clone().id,
            created_at: key.clone().created_at,
            updated_at: key.clone().updated_at,
        })
        .collect();
    return JsonResponse::send(200, Some(res), None);
}

async fn delete_api_key(
    Path(id): Path<String>,
    headers: HeaderMap,
    State(pool): State<DbPool>,
) -> AxumResponse<uuid::Uuid> {
    let api_key_id = uuid::Uuid::parse_str(&id).unwrap();
    let user_id = match extract_session_user_id::<String>(&headers) {
        Ok(id) => id,
        Err(err) => return JsonResponse::send(403, None, Some(err.to_string())),
    };

    match ApiKey::delete(&pool, &api_key_id, &user_id) {
        Ok(key) => JsonResponse::send(200, Some(key.id), None),
        Err(err) => JsonResponse::send(500, None, Some(err.to_string())),
    }
}

pub fn api_key_routes() -> Router<DbPool> {
    Router::new()
        .route("/", post(create_api_key))
        .route("/", get(find_api_keys))
        .route("/{id}", delete(delete_api_key))
        .layer(from_fn(session_middleware))
}
