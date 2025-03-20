use axum::Router;

use crate::db::DbPool;

use super::developers::exchange_developer_routes;

pub fn exchange_routes() -> Router<DbPool> {
    Router::new().nest("/developers", exchange_developer_routes())
}
