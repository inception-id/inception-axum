// @generated automatically by Diesel CLI.

diesel::table! {
    ai_system_prompts (id) {
        id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        product_name -> Varchar,
        prompt -> Varchar,
        name -> Varchar,
    }
}

diesel::table! {
    checkbot (id) {
        id -> Int4,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        instruction -> Varchar,
        ai_system_prompt -> Varchar,
        content -> Text,
        completion -> Text,
        updated_completion -> Text,
    }
}

diesel::table! {
    languages (id) {
        id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        title -> Varchar,
        #[max_length = 2]
        iso_639_1 -> Varchar,
    }
}

diesel::table! {
    translation (id) {
        id -> Int4,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        ai_system_prompt -> Varchar,
        content_language -> Nullable<Varchar>,
        target_language -> Varchar,
        content -> Text,
        completion -> Text,
        updated_completion -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        supertokens_user_id -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        email -> Varchar,
    }
}

diesel::joinable!(checkbot -> users (user_id));
diesel::joinable!(translation -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    ai_system_prompts,
    checkbot,
    languages,
    translation,
    users,
);
