use serde::Deserialize;

#[derive(Deserialize)]
pub struct SupertokensEmailVerificationTokenResponse {
    #[allow(dead_code)]
    status: String,
    pub token: String,
}
