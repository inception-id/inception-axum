use serde::{de::DeserializeOwned, Serialize};

use crate::{
    supertokens::response::{SupertokensEmailVerificationTokenResponse, SupertokensSignUpResponse},
    users::RegisterUserPayload,
};

use super::paths::SupertokensPath;
use std::{collections::HashMap, env};

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

    async fn request_supertokens<T, U>(path: SupertokensPath, json: &T) -> Result<U, reqwest::Error>
    where
        T: Serialize,
        U: DeserializeOwned,
    {
        let supertokens = Supertokens::new(path);
        let url = format!("{}{}", &supertokens.connection_uri, &supertokens.path);

        let client = reqwest::Client::new();

        client
            .post(url)
            .header("Authorization", &supertokens.api_key)
            .json(json)
            .send()
            .await?
            .json()
            .await
    }

    pub async fn sign_up(
        payload: &RegisterUserPayload,
    ) -> Result<SupertokensSignUpResponse, reqwest::Error> {
        let json_payload = serde_json::json!(payload);
        Self::request_supertokens(SupertokensPath::SignUp, &json_payload).await
    }

    pub async fn create_email_verification_token(
        user_id: &uuid::Uuid,
        email: &str,
    ) -> Result<SupertokensEmailVerificationTokenResponse, reqwest::Error> {
        let mut map = HashMap::new();
        map.insert("userId", user_id.to_string());
        map.insert("email", email.to_string());
        Self::request_supertokens(SupertokensPath::EmailVerificationToken, &map).await
    }
}
