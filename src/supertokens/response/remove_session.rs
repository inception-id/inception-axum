use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct SupertokensRemoveSessionResponse {
    pub status: String,
    sessionHandlesRevoked: Vec<String>,
}
