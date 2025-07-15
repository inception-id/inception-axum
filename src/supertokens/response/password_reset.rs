use serde::Deserialize;

#[derive(Deserialize)]
pub struct SupertokensPasswordResetTokenResponse {
    pub status: String,
    pub token: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct SupertokensPasswordResetTokenConsumeResponse {
    pub status: String,
    pub userId: Option<String>,
    #[allow(dead_code)]
    email: Option<String>,
}
