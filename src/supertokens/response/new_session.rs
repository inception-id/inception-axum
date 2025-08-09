use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct SupertokensNewSessionToken {
    token: String,
    expiry: u64,
    createdTime: u64,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct SupertokensNewSessionResponse {
    pub status: String,
    accessToken: Option<SupertokensNewSessionToken>,
    refreshToken: Option<SupertokensNewSessionToken>,
}
