use super::model::User;
use crate::{
    db::DbPool,
    mail::Mail,
    middleware::{api_key_middleware, AxumResponse, JsonResponse, RE_PHONE},
    supertokens::{Supertokens, SupertokensEmailVerificationResponse},
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
        None => return JsonResponse::send(400, None, Some(supertokens.status.replace("_", " "))),
    };

    let supertokens_user_id = match uuid::Uuid::parse_str(&recipe_user_id) {
        Ok(uuid) => uuid,
        Err(err) => return JsonResponse::send(500, None, Some(err.to_string())),
    };

    let user = match User::create(&pool, &supertokens_user_id, &payload) {
        Ok(res) => res,
        Err(err) => return JsonResponse::send(500, None, Some(err.to_string())),
    };

    let verification_token =
        match Supertokens::create_email_verification_token(&supertokens_user_id, &user.email).await
        {
            Ok(supertokens) => match supertokens.token {
                Some(token) => token,
                None => {
                    return JsonResponse::send(
                        400,
                        None,
                        Some(supertokens.status.replace("_", " ")),
                    )
                }
            },
            Err(err) => return JsonResponse::send(201, Some(user), Some(err.to_string())),
        };

    match Mail::send_register_verification_email(&user.email, &verification_token) {
        Ok(_) => JsonResponse::send(201, Some(user), None),
        Err(err) => return JsonResponse::send(201, Some(user), Some(err.to_string())),
    }
}

#[derive(Deserialize)]
pub struct VerifyUserPayload {
    token: String,
}

async fn verify_user(
    Json(payload): Json<VerifyUserPayload>,
) -> AxumResponse<SupertokensEmailVerificationResponse> {
    match Supertokens::verify_email(&payload.token).await {
        Ok(supertokens) => {
            if supertokens.status == "OK" {
                JsonResponse::send(200, Some(supertokens), None)
            } else {
                JsonResponse::send(400, None, Some(supertokens.status.replace("_", " ")))
            }
        }
        Err(err) => JsonResponse::send(500, None, Some(err.to_string())),
    }
}

#[derive(Deserialize, Validate)]
pub struct SendUserVerificationPayload {
    #[validate(email(message = "Invalid Email"))]
    email: String,
}

async fn send_user_verification(
    State(pool): State<DbPool>,
    Json(payload): Json<SendUserVerificationPayload>,
) -> AxumResponse<User> {
    let user = match User::find_by_email(&pool, &payload.email) {
        Ok(res) => res,
        Err(err) => return JsonResponse::send(400, None, Some(err.to_string())),
    };

    let supertokens_user_id = match user.supertokens_user_id {
        Some(uuid) => uuid,
        None => return JsonResponse::send(400, None, Some("User doesn't exist".to_string())),
    };

    let verification_token =
        match Supertokens::create_email_verification_token(&supertokens_user_id, &user.email).await
        {
            Ok(supertokens) => match supertokens.token {
                Some(token) => token,
                None => {
                    return JsonResponse::send(
                        400,
                        None,
                        Some(supertokens.status.replace("_", " ")),
                    )
                }
            },
            Err(err) => return JsonResponse::send(500, None, Some(err.to_string())),
        };

    match Mail::send_register_verification_email(&user.email, &verification_token) {
        Ok(_) => JsonResponse::send(200, None, None),
        Err(err) => return JsonResponse::send(500, None, Some(err.to_string())),
    }
}

pub fn user_routes() -> Router<DbPool> {
    Router::new()
        .route("/register", post(register))
        .route("/verify", post(verify_user))
        .route("/verify/send", post(send_user_verification))
        .layer(from_fn(api_key_middleware))
}
