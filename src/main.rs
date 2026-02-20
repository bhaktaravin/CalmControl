use axum::{
    Router,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
};
use serde_json::json;
use std::{env, sync::Arc};
use tower_sessions::{MemoryStore, SessionManagerLayer};

mod db;
mod handlers;
mod models;
mod state;
mod store;
mod templates;

use handlers::{auth, dashboard, newsletter, profile, sessions, videos};
use state::AppState;
use store::UserStore;

async fn health() -> impl IntoResponse {
    Json(json!({ "status": "ok", "service": "CalmControl" }))
}

async fn not_found() -> impl IntoResponse {
    let html = templates::not_found_page();
    (StatusCode::NOT_FOUND, axum::response::Html(html))
}

#[tokio::main]
async fn main() {
    let database_url =
        env::var("SQLITE_URL").unwrap_or_else(|_| "sqlite:calmcontrol.db".to_string());

    let pool = db::create_pool(&database_url)
        .await
        .expect("Failed to initialise database");

    let user_store = UserStore::new(pool);
    let app_state = Arc::new(AppState { user_store });

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store).with_secure(false);

    let app = Router::new()
        .route("/", get(dashboard::show_dashboard))
        .route(
            "/login",
            get(auth::show_login_page).post(auth::process_login),
        )
        .route("/logout", get(auth::logout))
        .route(
            "/register",
            get(auth::show_register_page).post(auth::process_register),
        )
        .route("/dashboard", get(dashboard::show_dashboard))
        .route("/profile", get(profile::show_profile))
        .route("/breathe", get(sessions::show_breathe))
        .route("/breathe/complete", post(sessions::complete_breathe))
        .route("/meditate", get(sessions::show_meditate))
        .route("/meditate/complete", post(sessions::complete_meditate))
        .route(
            "/journal",
            get(sessions::show_journal).post(sessions::submit_journal),
        )
        .route(
            "/videos",
            get(videos::show_videos).post(videos::create_video),
        )
        .route("/videos/new", get(videos::show_new_video))
        .route("/videos/:id", get(videos::show_video))
        .route("/newsletter", get(newsletter::show_newsletter))
        .route("/newsletter/:id", get(newsletter::show_article))
        .route(
            "/newsletter/subscribe",
            get(newsletter::show_subscribe).post(newsletter::process_subscribe),
        )
        .route(
            "/newsletter/unsubscribe",
            get(newsletter::process_unsubscribe),
        )
        .route(
            "/api/newsletter/subscribers",
            get(newsletter::api_get_subscribers),
        )
        .route(
            "/api/newsletter/article",
            post(newsletter::api_post_article),
        )
        .route("/health", get(health))
        .fallback(not_found)
        .with_state(app_state)
        .layer(session_layer);

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{port}");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("🌿 CalmControl running on http://{addr}");
    axum::serve(listener, app).await.unwrap();
}
