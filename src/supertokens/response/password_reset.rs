use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SupertokensPasswordResetTokenResponse {
    pub status: String,
    pub token: Option<String>,
}
