// @generated automatically by Diesel CLI.

diesel::table! {
    companies (id) {
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        legal_name -> Nullable<Varchar>,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        #[max_length = 255]
        phone -> Nullable<Varchar>,
        #[max_length = 255]
        website -> Nullable<Varchar>,
        #[max_length = 255]
        api_key -> Nullable<Varchar>,
        ip_address -> Jsonb,
    }
}

diesel::table! {
    company_users (user_id, company_id) {
        user_id -> Uuid,
        company_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
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
        is_super_admin -> Bool,
    }
}

diesel::joinable!(company_users -> companies (company_id));
diesel::joinable!(company_users -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    companies,
    company_users,
    users,
);
