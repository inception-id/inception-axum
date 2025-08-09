use serde::Serialize;

#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct SupertokensRefreshSessionRequest {
    refreshToken: String,
    enableAntiCsrf: bool,
    useDynamicSigningKey: bool,
}

impl SupertokensRefreshSessionRequest {
    pub fn new(refresh_token: &str) -> Self {
        Self {
            refreshToken: refresh_token.to_string(),
            enableAntiCsrf: false,
            useDynamicSigningKey: true,
        }
    }
}
