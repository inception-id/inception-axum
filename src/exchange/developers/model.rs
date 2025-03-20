use crate::db::DbPool;
use crate::schema::exchange_developers;
use diesel::{prelude::Queryable, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Queryable, Serialize)]
pub(super) struct ExchangeDeveloper {
    id: uuid::Uuid,
    user_id: uuid::Uuid,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    pub api_key: String,
    api_cost: f64,
    balance: f64,
}

impl ExchangeDeveloper {
    pub(super) fn create_api_key() -> String {
        let exp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Can't get current time")
            .as_millis();
        let token = encode(
            &Header::default(),
            &exp,
            &EncodingKey::from_secret("secret".as_ref()),
        )
        .expect("Fail to create token");
        let keys = token.split(".").collect::<Vec<&str>>();
        keys[1].to_string()
    }

    pub(super) fn create(
        pool: &DbPool,
        user_id: &uuid::Uuid,
        api_key: &str,
        api_cost: &f64,
    ) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        let values = (
            exchange_developers::user_id.eq(user_id),
            exchange_developers::api_key.eq(api_key),
            exchange_developers::api_cost.eq(api_cost),
        );

        diesel::insert_into(exchange_developers::table)
            .values(&values)
            .get_result(conn)
    }

    pub(super) fn find(pool: &DbPool, user_id: &uuid::Uuid) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        exchange_developers::table
            .filter(exchange_developers::user_id.eq(user_id))
            .get_result(conn)
    }
}
