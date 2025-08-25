use base64::prelude::*;
use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};
use diesel::{prelude::Queryable, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tracing_subscriber::fmt::format;

use crate::{db::DbPool, schema, users::RegisterUserPayload};
use std::io::ErrorKind;

#[derive(Queryable, Serialize, Clone, Deserialize)]
pub(super) struct ApiKey {
    pub id: uuid::Uuid,
    user_id: uuid::Uuid,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    api_key: String,
}

impl ApiKey {
    pub(super) fn create_key() -> Result<(String, String), std::io::Error> {
        let rand_uuid = uuid::Uuid::new_v4().to_string();
        let mut sha_hasher = Sha256::new();
        sha_hasher.update(rand_uuid);
        let sha_hash = sha_hasher.finalize();
        let hex_string = format!("{:x}", sha_hash);
        let base64_string = BASE64_STANDARD.encode(hex_string);
        match hash(&base64_string, DEFAULT_COST) {
            Ok(hashed) => Ok((base64_string, hashed)),
            Err(err) => {
                let new_err = std::io::Error::new(ErrorKind::Other, err.to_string());
                Err(new_err)
            }
        }
    }

    pub(super) fn create(
        pool: &DbPool,
        user_id: &uuid::Uuid,
        hashed_key: &str,
    ) -> QueryResult<ApiKey> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        let values = (
            schema::api_keys::user_id.eq(user_id),
            schema::api_keys::api_key.eq(hashed_key),
        );
        diesel::insert_into(schema::api_keys::table)
            .values(values)
            .get_result(conn)
    }
}
