use super::model::ApiKey;
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    middleware::from_fn,
    routing::{delete, post},
    Json, Router,
};
use serde::Deserialize;

use crate::{
    db::DbPool,
    middleware::{
        api_key_middleware, extract_session_user_id, session_middleware, AxumResponse, JsonResponse,
    },
    supertokens::{Supertokens, SupertokensNewSessionResponse, SupertokensRemoveSessionResponse},
};

async fn generate_api_key(headers: HeaderMap, State(pool): State<DbPool>) -> AxumResponse<String> {
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

pub fn api_key_routes() -> Router<DbPool> {
    Router::new()
        .route("/", post(generate_api_key))
        .layer(from_fn(session_middleware))
}
