use axum::{
    extract::{Json, State},
    routing::post,
    Router,
};

use crate::{
    db::DbPool,
    middleware::{AxumResponse, JsonResponse},
};

use super::model::User;

async fn register(
    State(pool): State<DbPool>,
    // Json(): State<Json>
) -> AxumResponse<u16> {
    JsonResponse::send(200, Some(200), None)
}

pub fn user_routes() -> Router<DbPool> {
    Router::new().route("/", post(register))
}
