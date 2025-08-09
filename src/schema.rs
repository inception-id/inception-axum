// @generated automatically by Diesel CLI.

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

diesel::table! {
    whatsapp_sessions (id) {
        id -> Uuid,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[max_length = 255]
        phone -> Varchar,
        is_ready -> Bool,
    }
}

diesel::joinable!(whatsapp_sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    users,
    whatsapp_sessions,
);
