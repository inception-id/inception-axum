use axum::{extract, http, middleware, response};
use serde::Serialize;

use crate::{
    middleware::{AxumResponse, JsonResponse},
    supertokens::Supertokens,
};

pub async fn session_middleware(
    req: extract::Request,
    next: middleware::Next,
) -> Result<response::Response, AxumResponse<String>> {
    let access_token = match req.headers().get("x-access-token") {
        Some(token) => token.to_str().unwrap_or(""),
        None => return Err(JsonResponse::send(403, None, None)),
    };
    match Supertokens::verify_session(access_token).await {
        Ok(session_verification) if session_verification.status == "OK" => {
            let user_id = match session_verification.session {
                Some(session) => session.userDataInJWT.id.to_string(),
                None => {
                    let res = JsonResponse::send(
                        500,
                        None,
                        Some("Fail to parse JWT user ID".to_string()),
                    );
                    return Err(res);
                }
            };
            let x_user_id = match http::HeaderValue::from_str(&user_id) {
                Ok(val) => val,
                Err(err) => {
                    let res = JsonResponse::send(500, None, Some(err.to_string()));
                    return Err(res);
                }
            };
            let mut new_req = req;
            new_req.headers_mut().insert("x-user-id", x_user_id);
            return Ok(next.run(new_req).await);
        }
        Ok(session) => {
            return Err(JsonResponse::send(
                403,
                None,
                Some(session.status.replace("_", " ")),
            ))
        }
        Err(err) => return Err(JsonResponse::send(500, None, Some(err.to_string()))),
    };
}

pub fn extract_session_user_id<T: Serialize>(
    header_map: &http::HeaderMap,
) -> Result<uuid::Uuid, AxumResponse<T>> {
    let header_user_id = match header_map.get("x-user-id") {
        Some(id) => id,
        None => {
            return Err(JsonResponse::send(401, None, None));
        }
    };
    let user_id = match header_user_id.to_str() {
        Ok(id) => id,
        Err(err) => {
            return Err(JsonResponse::send(500, None, Some(err.to_string())));
        }
    };
    match uuid::Uuid::parse_str(user_id) {
        Ok(id) => Ok(id),
        Err(err) => Err(JsonResponse::send(500, None, Some(err.to_string()))),
    }
}
