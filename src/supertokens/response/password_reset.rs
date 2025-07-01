use serde::Deserialize;

#[derive(Deserialize)]
pub struct SupertokensPasswordResetTokenResponse {
    pub status: String,
    pub token: Option<String>,
}
