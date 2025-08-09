use serde::{Deserialize, Serialize};

use crate::users::User;

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct SupertokensVerifySession {
    pub userDataInJWT: User,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct SupertokensVerifySessionResponse {
    pub status: String,
    pub session: Option<SupertokensVerifySession>,
}
