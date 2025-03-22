use crate::middleware::extract_header_user_id;
use crate::{db::DbPool, middleware::ApiResponse};
use axum::http::{HeaderMap, StatusCode};
use axum::routing::{get, post};
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

pub async fn recreate_exchange_api_key_route(
    State(pool): State<DbPool>,
    headers: HeaderMap,
) -> (
    StatusCode,
    Json<ApiResponse<CreateExchangeDeveloperResponse>>,
) {
    let user_id = extract_header_user_id(headers).expect("Can't extract user id");

    let new_api_key = ExchangeDeveloper::create_api_key();
    let encrypted_key = hash(&new_api_key, DEFAULT_COST).expect("Fail to create hashed key");
    match ExchangeDeveloper::update_api_key(&pool, &user_id, &encrypted_key) {
        Ok(_) => {
            let response = CreateExchangeDeveloperResponse {
                api_key: new_api_key,
            };
            ApiResponse::reply(StatusCode::CREATED, Some(response), "created")
        }
        Err(err) => ApiResponse::reply(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()),
    }
}

#[derive(Serialize)]
pub struct FindExchangeDeveloperResponse {
    id: uuid::Uuid,
    user_id: uuid::Uuid,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    api_cost: f64,
    balance: f64,
}

pub async fn find_exchange_developer_route(
    State(pool): State<DbPool>,
    headers: HeaderMap,
) -> (StatusCode, Json<ApiResponse<FindExchangeDeveloperResponse>>) {
    let user_id = extract_header_user_id(headers).expect("Could not extract user id");
    match ExchangeDeveloper::find(&pool, &user_id) {
        Ok(dev) => {
            let response = FindExchangeDeveloperResponse {
                id: dev.id,
                user_id: dev.user_id,
                created_at: dev.created_at,
                updated_at: dev.updated_at,
                api_cost: dev.api_cost,
                balance: dev.balance,
            };
            ApiResponse::reply(StatusCode::OK, Some(response), "found")
        }
        Err(err) => ApiResponse::reply(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()),
    }
}

pub fn exchange_developer_routes() -> Router<DbPool> {
    Router::new()
        .route("/create", post(create_exchange_developer_route))
        .route("/recreate", post(recreate_exchange_api_key_route))
        .route("/find", get(find_exchange_developer_route))
}
