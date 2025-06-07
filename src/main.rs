mod db;
mod middleware;
mod schema;
mod supertokens;
mod users;

use crate::db::build_db_pool;
use crate::users::user_routes;
use axum::Router;
use sentry_tower::{NewSentryLayer, SentryHttpLayer};
use std::env;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let pool = build_db_pool();
    let cors = CorsLayer::permissive();

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
    let app = Router::new()
        .nest("/users", user_routes())
        .with_state(pool)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .layer(NewSentryLayer::new_from_top())
        .layer(SentryHttpLayer::new().enable_transaction());

    // run our app with hyper, listening globally on env port
    let host_addr = env::var("HOST_ADDRESS").expect("Missing HOST_ADDRESS");
    let listener = tokio::net::TcpListener::bind(&host_addr).await.unwrap();

    println!("Server started at {}", host_addr);
    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}
