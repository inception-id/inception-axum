use crate::{db::DbPool, enums::CompanyUserPermission, schema};
use diesel::{prelude::Queryable, ExpressionMethods, QueryResult, RunQueryDsl};
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct CompanyUser {
    company_id: uuid::Uuid,
    user_id: uuid::Uuid,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    permission: CompanyUserPermission,
}

impl CompanyUser {
    fn create(
        pool: DbPool,
        company_id: &uuid::Uuid,
        user_id: &uuid::Uuid,
        permission: &CompanyUserPermission,
    ) -> QueryResult<CompanyUser> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        let values = (
            schema::companies_users::company_id.eq(company_id),
            schema::companies_users::user_id.eq(user_id),
            schema::companies_users::permission.eq(permission),
        );
        diesel::insert_into(schema::companies_users::table)
            .values(values)
            .get_result(conn)
    }
}
