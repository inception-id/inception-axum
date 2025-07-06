use serde::Serialize;

use crate::users::User;

#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct SupertokensNewSessionRequest {
    userId: String,
    userDataInJWT: User,
    userDataInDatabase: User,
    enableAntiCsrf: bool,
    useDynamicSigningKey: bool,
}

impl SupertokensNewSessionRequest {
    pub fn new(user_id: &str, user: &User) -> Self {
        Self {
            userId: user_id.to_string(),
            userDataInJWT: user.clone(),
            userDataInDatabase: user.clone(),
            enableAntiCsrf: false,
            useDynamicSigningKey: false,
        }
    }
}
