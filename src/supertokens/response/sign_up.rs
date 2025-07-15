use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct SupertokensSignUpResponse {
    pub status: String,
    pub recipeUserId: Option<String>,
}
