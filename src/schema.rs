// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "whatsapp_message_type"))]
    pub struct WhatsappMessageType;
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

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::WhatsappMessageType;

    whatsapp_messages (id) {
        id -> Uuid,
        session_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[max_length = 255]
        target_phone -> Varchar,
        message_type -> WhatsappMessageType,
        text_message -> Nullable<Text>,
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
        is_deleted -> Bool,
    }
}

diesel::joinable!(whatsapp_messages -> whatsapp_sessions (session_id));
diesel::joinable!(whatsapp_sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    users,
    whatsapp_messages,
    whatsapp_sessions,
);
