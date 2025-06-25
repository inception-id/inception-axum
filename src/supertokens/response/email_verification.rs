use serde::Deserialize;

#[derive(Deserialize)]
pub struct SupertokensEmailVerificationTokenResponse {
    #[allow(dead_code)]
    status: String,
    pub token: String,
}

#[derive(Deserialize)]
pub struct SupertokensEmailVerificationResponse {
    #[allow(dead_code)]
    status: String,
    userId: String,
    email: String,
}
