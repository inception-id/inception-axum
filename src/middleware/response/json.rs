use axum::{http::StatusCode, Json};
use serde::Serialize;

pub type AxumResponse<T> = (StatusCode, Json<JsonResponse<T>>);

#[derive(Debug, Serialize)]
pub struct JsonResponse<T> {
    status: u16,
    data: Option<T>,
    message: String,
}

impl<T> JsonResponse<T> {
    fn new(status: u16, data: Option<T>, message: String) -> Self {
        Self {
            status,
            data,
            message,
        }
    }
    pub fn send(status: u16, data: Option<T>, message: Option<String>) -> AxumResponse<T> {
        let msg = match message {
            Some(msg) => msg,
            None => match StatusCode::from_u16(status) {
                Ok(status_code) => status_code.to_string(),
                Err(err) => err.to_string(),
            },
        };
        let response = Self::new(status, data, msg);
        (
            StatusCode::from_u16(status).expect("Invalid status code"),
            Json(response),
        )
    }
}
