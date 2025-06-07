use super::model::User;
use crate::{
    db::DbPool,
    middleware::{AxumResponse, JsonResponse},
    supertokens::{self, Supertokens},
};
use axum::{
    extract::{Json, State},
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct RegisterUserPayload {
    pub email: String,
    pub phone: Option<String>,
    password: String,
}

async fn register(
    State(pool): State<DbPool>,
    Json(payload): Json<RegisterUserPayload>,
) -> AxumResponse<User> {
    // TODO: Validatte payload
    let supertokens = match Supertokens::sign_up(&payload).await {
        Ok(res) => res,
        Err(err) => return JsonResponse::send(500, None, Some(err.to_string())),
    };

    let recipe_user_id = match supertokens.recipeUserId {
        Some(id) => id,
        None => return JsonResponse::send(400, None, Some(supertokens.status)),
    };

    let supertokens_user_id = match uuid::Uuid::parse_str(&recipe_user_id) {
        Ok(uuid) => Some(uuid),
        Err(err) => return JsonResponse::send(500, None, Some(err.to_string())),
    };
    let user = match User::create(&pool, &supertokens_user_id, &payload) {
        Ok(res) => res,
        Err(err) => return JsonResponse::send(500, None, Some(err.to_string())),
    };
    JsonResponse::send(200, Some(user), None)
}

pub fn user_routes() -> Router<DbPool> {
    Router::new().route("/", post(register))
}
