use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SupertokensEmailVerificationTokenResponse {
    #[allow(dead_code)]
    status: String,
    pub token: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct SupertokensEmailVerificationResponse {
    pub status: String,
    userId: Option<String>,
    email: Option<String>,
}
