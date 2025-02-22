use crate::language_ai::SharedStoragePermission;
use crate::schema::sql_types;
use diesel::sql_types::{Integer, Text, Uuid, VarChar, Nullable};
use diesel::QueryableByName;
use serde::Serialize;

#[derive(QueryableByName, Debug, Serialize)]
pub(crate) struct SharedTranslationStorageJoinTranslationStorage {
    #[diesel(sql_type=Integer)]
    shared_storage_id: i32,
    #[diesel(sql_type=Integer)]
    storage_id: i32,
    #[diesel(sql_type=Uuid)]
    owner_id: uuid::Uuid,
    #[diesel(sql_type=VarChar)]
    owner_email: String,
    #[diesel(sql_type= sql_types::SharedStoragePermission)]
    permission: SharedStoragePermission,
    #[diesel(sql_type=VarChar)]
    content_language: String,
    #[diesel(sql_type=VarChar)]
    target_language: String,
    #[diesel(sql_type=Nullable<VarChar>)]
    title: Option<String>,
    #[diesel(sql_type=Text)]
    content: String,
    #[diesel(sql_type=Text)]
    updated_completion: String,
}
