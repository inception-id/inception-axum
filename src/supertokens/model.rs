use crate::{supertokens::response::SupertokensSignUpResponse, users::RegisterUserPayload};

use super::paths::SupertokensPath;
use std::env;

pub struct Supertokens {
    connection_uri: String,
    api_key: String,
    path: String,
}

impl Supertokens {
    fn new(path: SupertokensPath) -> Self {
        let connection_uri =
            env::var("SUPERTOKENS_CONNECTION_URI").expect("Missing SUPERTOKENS_CONNECTION_URI");

        let api_key = env::var("SUPERTOKENS_API_KEY").expect("Missing SUPERTOKENS_API_KEY");

        Supertokens {
            connection_uri,
            api_key,
            path: path.to_string(),
        }
    }

    pub async fn sign_up(
        payload: &RegisterUserPayload,
    ) -> Result<SupertokensSignUpResponse, reqwest::Error> {
        let supertokens = Supertokens::new(SupertokensPath::SignUp);
        let url = format!("{}{}", &supertokens.connection_uri, &supertokens.path);

        let json_payload = serde_json::json!(payload);
        let client = reqwest::Client::new();

        client
            .post(url)
            .header("Authorization", &supertokens.api_key)
            .json(&json_payload)
            .send()
            .await?
            .json()
            .await
    }
}
