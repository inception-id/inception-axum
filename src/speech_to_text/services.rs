use super::routes::CreateTranscriptionPayload;
use crate::db::DbPool;
use crate::schema::speech_to_text;
use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl};
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub(super) struct SpeechToText {
    id: i32,
    user_id: uuid::Uuid,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    audio_url: String,
    transcription_text: String,
    language: Option<String>,
}

impl SpeechToText {
    pub(super) fn create_transcription(
        pool: &DbPool,
        user_id: &uuid::Uuid,
        payload: &CreateTranscriptionPayload,
    ) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        diesel::insert_into(speech_to_text::table)
            .values((speech_to_text::user_id.eq(user_id), payload))
            .get_result(conn)
    }

    pub(super) fn find_transcription_history(
        pool: &DbPool,
        user_id: &uuid::Uuid,
    ) -> QueryResult<Vec<Self>> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        speech_to_text::table
            .filter(speech_to_text::user_id.eq(user_id))
            .order_by(speech_to_text::id.desc())
            .limit(10)
            .get_results(conn)
    }
}
