use diesel::{prelude::Queryable, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use serde::Serialize;

use crate::{db::DbPool, schema, users::RegisterUserPayload};

#[derive(Queryable, Serialize)]
pub(super) struct User {
    pub id: uuid::Uuid,
    pub supertokens_user_id: Option<uuid::Uuid>,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    pub email: String,
    phone: Option<String>,
}

impl User {
    pub(super) fn create(
        pool: &DbPool,
        supertokens_user_id: &uuid::Uuid,
        payload: &RegisterUserPayload,
    ) -> QueryResult<User> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        let values = (
            schema::users::supertokens_user_id.eq(supertokens_user_id),
            schema::users::email.eq(&payload.email),
            schema::users::phone.eq(&payload.phone),
        );
        diesel::insert_into(schema::users::table)
            .values(values)
            .get_result(conn)
    }

    pub(super) fn find_by_email(pool: &DbPool, email: &str) -> QueryResult<User> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        schema::users::table
            .filter(schema::users::email.eq(email))
            .get_result(conn)
    }

    pub(super) fn find_by_supertokens_id(
        pool: &DbPool,
        supertokens_id: &uuid::Uuid,
    ) -> QueryResult<User> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        schema::users::table
            .filter(schema::users::supertokens_user_id.eq(supertokens_id))
            .get_result(conn)
    }
}
