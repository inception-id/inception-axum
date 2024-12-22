use super::services::AiSystemPrompt;
use crate::db::DbPool;
use crate::middleware::ApiResponse;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::routing::{get, post, delete};
use axum::{Json, Router};
use serde::Deserialize;

type AiSystemPromptResponse = (StatusCode, Json<ApiResponse<AiSystemPrompt>>);

#[derive(Debug, Deserialize)]
struct CreateAiSystemPromptPayload {
    product_name: String,
    prompt: String,
    name: String
}

async fn create_ai_system_prompt_route(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateAiSystemPromptPayload>,
) -> AiSystemPromptResponse {
    let ai_system_prompt_creation =
        AiSystemPrompt::create_ai_system_prompt(&pool, &payload.product_name, &payload.prompt, &payload.name);

    match ai_system_prompt_creation {
        Ok(ai_system_prompt) => ApiResponse::new(
            StatusCode::CREATED,
            Some(ai_system_prompt),
            "Created",
        )
        .send(),
        Err(error) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &error.to_string()).send()
        }
    }
}

#[derive(Deserialize)]
struct FindAiSystemPromptQuery {
    product_name: Option<String>,
}
async fn find_ai_system_prompts_route(
    State(pool): State<DbPool>,
    query: Query<FindAiSystemPromptQuery>,
) -> (StatusCode, Json<ApiResponse<Vec<AiSystemPrompt>>>) {
    let ai_system_prompts = AiSystemPrompt::find_ai_system_prompts(&pool, &query.product_name);

    match ai_system_prompts {
        Ok(ai_system_prompts) => {
            ApiResponse::new(StatusCode::CREATED, Some(ai_system_prompts), "System prompt created").send()
        }
        Err(error) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &error.to_string()).send()
        }
    }
}

#[derive(Deserialize)]
struct DeleteAiSystemPromptQuery {
    id: i32
}

async fn delete_ai_system_prompt_route(
    State(pool): State<DbPool>,
    query: Query<DeleteAiSystemPromptQuery>,
) -> AiSystemPromptResponse {
   let deleted_ai_system_prompt = AiSystemPrompt::delete_ai_system_prompts(&pool, &query.id); 
    
    match deleted_ai_system_prompt { 
        Ok(ai_system_prompt) => {
            ApiResponse::new(StatusCode::OK, Some(ai_system_prompt), "System prompt deleted").send()
        },
        Err(error) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &error.to_string()).send()
        }
    }
}

pub fn ai_system_prompt_routes() -> Router<DbPool> {
    Router::new()
        .route("/create", post(create_ai_system_prompt_route))
        .route("/find-all", get(find_ai_system_prompts_route)).route("/delete", delete(delete_ai_system_prompt_route))
}
