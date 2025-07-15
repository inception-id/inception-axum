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
