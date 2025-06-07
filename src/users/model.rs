use diesel::{prelude::Queryable, ExpressionMethods, QueryResult, RunQueryDsl};
use serde::Serialize;

use crate::{db::DbPool, schema, users::RegisterUserPayload};

#[derive(Queryable, Serialize)]
pub(super) struct User {
    id: uuid::Uuid,
    supertokens_user_id: Option<uuid::Uuid>,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    email: String,
    phone: Option<String>,
    is_super_admin: bool,
}

impl User {
    pub(super) fn create(
        pool: &DbPool,
        supertokens_user_id: &Option<uuid::Uuid>,
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
}
