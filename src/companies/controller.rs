use super::model::Company;
use crate::{
    companies_users::CompanyUser,
    db::DbPool,
    enums::CompanyUserPermission,
    middleware::{
        extract_session_user_id, session_middleware, AxumResponse, JsonResponse, RE_COMPANY_PHONE,
    },
};
use axum::{
    extract::{Json, State},
    http::HeaderMap,
    middleware::from_fn,
    routing::post,
    Router,
};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateCompanyPayload {
    name: String,
    #[validate(length(
        min = 8,
        max = 13,
        message = "Phone must be between 8 to 13 characters"
    ))]
    #[validate(regex(
        path = "RE_COMPANY_PHONE",
        message = "Phone must  contains only numbers"
    ))]
    phone: String,
}

async fn create_company(
    State(pool): State<DbPool>,
    headers: HeaderMap,
    Json(payload): Json<CreateCompanyPayload>,
) -> AxumResponse<Company> {
    let user_id = match extract_session_user_id::<Company>(&headers) {
        Ok(user_id) => user_id,
        Err(err) => return err,
    };

    let api_key = match Company::generate_api_key() {
        Ok(api_key) => api_key,
        Err(err) => return JsonResponse::send(500, None, Some(err.to_string())),
    };
    let company = match Company::create(&pool, &payload.name, &payload.phone, &api_key) {
        Ok(company) => company,
        Err(err) => return JsonResponse::send(500, None, Some(err.to_string())),
    };

    match CompanyUser::create(&pool, &company.id, &user_id, &CompanyUserPermission::Owner) {
        Ok(_) => JsonResponse::send(201, Some(company), None),
        Err(err) => JsonResponse::send(500, None, Some(err.to_string())),
    }
}

pub fn company_routes() -> Router<DbPool> {
    Router::new()
        .route("/", post(create_company))
        .layer(from_fn(session_middleware))
}
