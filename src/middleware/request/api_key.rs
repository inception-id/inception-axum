use crate::middleware::{AxumResponse, JsonResponse};

use axum::{extract::Request, middleware::Next, response::Response};
use std::env;

pub async fn api_key_middleware(
    req: Request,
    next: Next,
) -> Result<Response, AxumResponse<String>> {
    let env_api_key = env::var("API_KEY").expect("Missing API_KEY");
    let header_api_key_option = req
        .headers()
        .get("x-api-key")
        .and_then(|header| header.to_str().ok());

    match header_api_key_option {
        Some(header_api_key) if header_api_key == env_api_key => Ok(next.run(req).await),
        _ => {
            let err =
                JsonResponse::send(401, None, Some("Missing or invalid x-api-key".to_string()));
            Err(err)
        }
    }
}
