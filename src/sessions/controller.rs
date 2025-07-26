use axum::{middleware::from_fn, routing::post, Json, Router};
use serde::Deserialize;

use crate::{
    db::DbPool,
    middleware::{api_key_middleware, AxumResponse, JsonResponse},
    supertokens::{Supertokens, SupertokensNewSessionResponse},
};

#[derive(Deserialize)]
pub struct VerifySessionPayload {
    token: String,
}

async fn verify_session(Json(payload): Json<VerifySessionPayload>) -> AxumResponse<bool> {
    match Supertokens::verify_session(&payload.token).await {
        Ok(supertokens) if supertokens.status == "OK" => JsonResponse::send(200, Some(true), None),
        Ok(supertokens) => {
            JsonResponse::send(403, Some(false), Some(supertokens.status.replace("_", " ")))
        }
        Err(err) => JsonResponse::send(500, Some(false), Some(err.to_string())),
    }
}

#[derive(Deserialize)]
pub struct RefreshSessionPayload {
    refresh_token: String,
}

async fn refresh_session(
    Json(payload): Json<RefreshSessionPayload>,
) -> AxumResponse<SupertokensNewSessionResponse> {
    match Supertokens::refresh_session(&payload.refresh_token).await {
        Ok(supertokens) if supertokens.status == "OK" => {
            JsonResponse::send(200, Some(supertokens), None)
        }
        Ok(supertokens) => {
            JsonResponse::send(400, None, Some(supertokens.status.replace("_", " ")))
        }
        Err(err) => JsonResponse::send(500, None, Some(err.to_string())),
    }
}

pub fn session_routes() -> Router<DbPool> {
    Router::new()
        .route("/verify", post(verify_session))
        .route("/refresh", post(refresh_session))
        .layer(from_fn(api_key_middleware))
}
