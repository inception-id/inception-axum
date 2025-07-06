use serde::Deserialize;

#[derive(Deserialize)]
pub struct SupertokensLoginMethod {
    pub verified: bool,
    pub email: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct SupertokensSignInUser {
    pub id: String,
    pub loginMethods: Vec<SupertokensLoginMethod>,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct SupertokensSignInResponse {
    pub status: String,
    pub user: Option<SupertokensSignInUser>,
}
