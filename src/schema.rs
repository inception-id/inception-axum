// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "whatsapp_environment"))]
    pub struct WhatsappEnvironment;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "whatsapp_message_status"))]
    pub struct WhatsappMessageStatus;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "whatsapp_payment_status"))]
    pub struct WhatsappPaymentStatus;
}

diesel::table! {
    api_keys (id) {
        id -> Uuid,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[max_length = 255]
        api_key -> Varchar,
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

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::WhatsappEnvironment;
    use super::sql_types::WhatsappMessageStatus;

    whatsapp_messages (id) {
        id -> Uuid,
        session_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[max_length = 255]
        target_phone -> Varchar,
        text_message -> Nullable<Text>,
        #[max_length = 255]
        country_code -> Varchar,
        environment -> WhatsappEnvironment,
        status -> WhatsappMessageStatus,
        media_url -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::WhatsappEnvironment;
    use super::sql_types::WhatsappMessageStatus;

    whatsapp_notifications (id) {
        id -> Uuid,
        session_id -> Uuid,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[max_length = 255]
        target_phone -> Varchar,
        text_message -> Nullable<Text>,
        environment -> WhatsappEnvironment,
        #[max_length = 255]
        country_code -> Varchar,
        status -> WhatsappMessageStatus,
        media_url -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::WhatsappPaymentStatus;

    whatsapp_payments (id) {
        id -> Uuid,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        payment_status -> WhatsappPaymentStatus,
        amount -> Numeric,
        items -> Jsonb,
        doku_request -> Nullable<Jsonb>,
        doku_response -> Nullable<Jsonb>,
        paid_at -> Nullable<Timestamp>,
        year -> Nullable<Int4>,
        month -> Nullable<Int4>,
        doku_notif -> Jsonb,
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
        hourly_limit -> Int4,
        daily_limit -> Int4,
        is_disconnected -> Bool,
    }
}

diesel::joinable!(api_keys -> users (user_id));
diesel::joinable!(whatsapp_messages -> whatsapp_sessions (session_id));
diesel::joinable!(whatsapp_notifications -> users (user_id));
diesel::joinable!(whatsapp_notifications -> whatsapp_sessions (session_id));
diesel::joinable!(whatsapp_payments -> users (user_id));
diesel::joinable!(whatsapp_sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    api_keys,
    users,
    whatsapp_messages,
    whatsapp_notifications,
    whatsapp_payments,
    whatsapp_sessions,
);
