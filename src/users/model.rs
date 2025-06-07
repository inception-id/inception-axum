pub struct User {
    id: uuid::Uuid,
    supertokens_user_id: Option<uuid::Uuid>,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    email: String,
    phone: Option<String>,
    is_super_admin: bool,
}
