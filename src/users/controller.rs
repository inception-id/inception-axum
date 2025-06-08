use super::model::User;
use crate::{
    db::DbPool,
    middleware::{api_key_middleware, AxumResponse, JsonResponse, RE_PHONE},
    supertokens::Supertokens,
};

use axum::{
    extract::{Json, State},
    middleware::from_fn,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct RegisterUserPayload {
    #[validate(email(message = "Invalid Email"))]
    pub email: String,
    #[validate(length(
        min = 10,
        max = 13,
        message = "Phone must be between 10 to 13 characters"
    ))]
    #[validate(regex(
        path = "RE_PHONE",
        message = "Phone must start with 8 and contains only numbers"
    ))]
    pub phone: Option<String>,
    password: String,
}

async fn register(
    State(pool): State<DbPool>,
    Json(payload): Json<RegisterUserPayload>,
) -> AxumResponse<User> {
    match payload.validate() {
        Ok(_) => (),
        Err(err) => return JsonResponse::send(400, None, Some(err.to_string())),
    };

    let supertokens = match Supertokens::sign_up(&payload).await {
        Ok(res) => res,
        Err(err) => return JsonResponse::send(500, None, Some(err.to_string())),
    };

    let recipe_user_id = match supertokens.recipeUserId {
        Some(id) => id,
        None => return JsonResponse::send(400, None, Some(supertokens.status)),
    };

    let supertokens_user_id = match uuid::Uuid::parse_str(&recipe_user_id) {
        Ok(uuid) => uuid,
        Err(err) => return JsonResponse::send(500, None, Some(err.to_string())),
    };
    let user = match User::create(&pool, &supertokens_user_id, &payload) {
        Ok(res) => res,
        Err(err) => return JsonResponse::send(500, None, Some(err.to_string())),
    };
    // TODO: Send smtp email
    let verification_token =
        match Supertokens::create_email_verification_token(&supertokens_user_id, &user.email).await
        {
            Ok(supertokens) => supertokens.token,
            Err(err) => return JsonResponse::send(500, None, Some(err.to_string())),
        };
    JsonResponse::send(200, Some(user), None)
}

pub fn user_routes() -> Router<DbPool> {
    Router::new()
        .route("/", post(register))
        .layer(from_fn(api_key_middleware))
}
