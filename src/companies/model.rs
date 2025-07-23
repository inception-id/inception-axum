use crate::{db::DbPool, schema};
use base64::{prelude::BASE64_STANDARD, Engine};
use bcrypt::{hash, BcryptError, DEFAULT_COST};
use diesel::{prelude::Queryable, ExpressionMethods, QueryResult, RunQueryDsl};
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Company {
    pub id: uuid::Uuid,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    name: String,
    phone: String,
    api_key: String,
}

impl Company {
    pub fn generate_api_key() -> Result<String, BcryptError> {
        let new_uuid = uuid::Uuid::new_v4().to_string();
        let base64_uuid = BASE64_STANDARD.encode(new_uuid);
        hash(base64_uuid, DEFAULT_COST)
    }

    pub fn create(pool: &DbPool, name: &str, phone: &str, api_key: &str) -> QueryResult<Company> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        let values = (
            schema::companies::name.eq(name),
            schema::companies::phone.eq(phone),
            schema::companies::api_key.eq(api_key),
        );
        diesel::insert_into(schema::companies::table)
            .values(values)
            .get_result(conn)
    }
}
