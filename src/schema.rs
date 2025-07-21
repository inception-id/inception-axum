// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "companies_users_permission"))]
    pub struct CompaniesUsersPermission;
}

diesel::table! {
    companies (id) {
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        name -> Varchar,
        #[max_length = 255]
        phone -> Varchar,
        #[max_length = 255]
        api_key -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::CompaniesUsersPermission;

    companies_users (company_id, user_id) {
        company_id -> Uuid,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        permission -> CompaniesUsersPermission,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        supertokens_user_id -> Nullable<Uuid>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        phone -> Nullable<Varchar>,
    }
}

diesel::joinable!(companies_users -> companies (company_id));
diesel::joinable!(companies_users -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    companies,
    companies_users,
    users,
);
