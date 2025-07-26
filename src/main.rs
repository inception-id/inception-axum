mod companies;
mod companies_users;
mod db;
mod enums;
mod mail;
mod middleware;
mod schema;
mod sessions;
mod supertokens;
mod users;

use crate::companies::company_routes;
use crate::db::build_db_pool;
use crate::sessions::session_routes;
use crate::users::user_routes;

use axum::http::HeaderValue;
use axum::Router;
use sentry_tower::{NewSentryLayer, SentryHttpLayer};
use std::env;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let app_env = env::var("APP_ENV");
    let cors = match app_env {
        Ok(env) if env == "production" => {
            CorsLayer::new().allow_origin("https://inception.id".parse::<HeaderValue>().unwrap())
        }
        _ => CorsLayer::permissive(),
    };

    let tracing_format = "tower_http::trace::make_span=debug,tower_http::trace::on_response=debug,tower_http::trace::on_request=debug";
    let tracing_filter = tracing_subscriber::EnvFilter::new(tracing_format);
    tracing_subscriber::fmt()
        .with_env_filter(tracing_filter)
        .init();

    let sentry_url = env::var("SENTRY_URL").expect("Missing SENTRY_URL");
    let _guard = sentry::init((
        sentry_url,
        sentry::ClientOptions {
            release: sentry::release_name!(),
            traces_sample_rate: 1.0,
            ..Default::default()
        },
    ));

    // build our application with a route
    let pool = build_db_pool();
    let app = Router::new()
        .nest("/users", user_routes())
        .nest("/companies", company_routes())
        .nest("/sessions", session_routes())
        .with_state(pool)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .layer(NewSentryLayer::new_from_top())
        .layer(SentryHttpLayer::new().enable_transaction());

    // run our app, listening globally on env port
    let host_addr = env::var("HOST_ADDRESS").expect("Missing HOST_ADDRESS");
    let listener = tokio::net::TcpListener::bind(&host_addr).await.unwrap();

    println!("Server started at {}", host_addr);
    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}
