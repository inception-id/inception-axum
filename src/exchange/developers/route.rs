use crate::middleware::extract_header_user_id;
use crate::{db::DbPool, middleware::ApiResponse};
use axum::http::{HeaderMap, StatusCode};
use axum::routing::post;
use axum::Router;
use axum::{extract::State, Json};
use bcrypt::{hash, DEFAULT_COST};
use serde::{Deserialize, Serialize};

use super::model::ExchangeDeveloper;

#[derive(Deserialize)]
pub(super) struct CreateExchangeDeveloperRequest {
    api_cost: f64,
}

#[derive(Serialize)]
pub struct CreateExchangeDeveloperResponse {
    api_key: String,
}

pub async fn create_exchange_developer_route(
    State(pool): State<DbPool>,
    headers: HeaderMap,
    Json(payload): Json<CreateExchangeDeveloperRequest>,
) -> (
    StatusCode,
    Json<ApiResponse<CreateExchangeDeveloperResponse>>,
) {
    let user_id = extract_header_user_id(headers).expect("Can't extract user id");

    match ExchangeDeveloper::find(&pool, &user_id) {
        Ok(_) => ApiResponse::reply(StatusCode::BAD_REQUEST, None, "Api key already exists"),
        Err(_) => {
            let api_key = ExchangeDeveloper::create_api_key();
            let encrypted_key = hash(&api_key, DEFAULT_COST).expect("Fail to create hashed key");
            match ExchangeDeveloper::create(&pool, &user_id, &encrypted_key, &payload.api_cost) {
                Ok(_) => {
                    let response = CreateExchangeDeveloperResponse { api_key };
                    ApiResponse::reply(StatusCode::CREATED, Some(response), "created")
                }
                Err(err) => {
                    ApiResponse::reply(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string())
                }
            }
        }
    }
}

pub fn exchange_developer_routes() -> Router<DbPool> {
    Router::new().route("/create", post(create_exchange_developer_route))
}
