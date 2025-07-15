use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SupertokensEmailVerificationTokenResponse {
    pub status: String,
    pub token: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct SupertokensEmailVerificationResponse {
    pub status: String,
    userId: Option<String>,
    email: Option<String>,
}
