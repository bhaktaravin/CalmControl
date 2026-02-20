use axum::{
    extract::{Form, Path, State},
    response::{Html, IntoResponse, Redirect, Response},
};
use serde::Deserialize;
use std::sync::Arc;
use tower_sessions::Session;

use crate::{handlers::auth::SESSION_USER_KEY, state::AppState, templates};

// ── Forms ──────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct VideoForm {
    pub title: String,
    pub description: String,
    pub video_url: String,
    pub thumbnail_url: String,
    pub category: String,
}

// ── Helpers ────────────────────────────────────────────────────────────────────

async fn require_user(
    session: &Session,
    state: &Arc<AppState>,
) -> Result<crate::models::user::User, Response> {
    let email = session
        .get::<String>(SESSION_USER_KEY)
        .await
        .ok()
        .flatten()
        .ok_or_else(|| Redirect::to("/login").into_response())?;

    state
        .user_store
        .find_by_email(&email)
        .await
        .ok_or_else(|| Redirect::to("/login").into_response())
}

// ── Handlers ───────────────────────────────────────────────────────────────────

pub async fn show_videos(session: Session, State(state): State<Arc<AppState>>) -> Response {
    if let Err(r) = require_user(&session, &state).await {
        return r;
    }

    let videos = state.user_store.get_all_videos().await;
    Html(templates::videos_page(&videos)).into_response()
}

pub async fn show_new_video(session: Session, State(state): State<Arc<AppState>>) -> Response {
    if let Err(r) = require_user(&session, &state).await {
        return r;
    }

    Html(templates::new_video_page(None)).into_response()
}

pub async fn create_video(
    session: Session,
    State(state): State<Arc<AppState>>,
    Form(form): Form<VideoForm>,
) -> Response {
    let user = match require_user(&session, &state).await {
        Ok(u) => u,
        Err(r) => return r,
    };

    let title = form.title.trim().to_string();
    let video_url = form.video_url.trim().to_string();
    let category = form.category.trim().to_string();

    if title.is_empty() {
        return Html(templates::new_video_page(Some("Title is required."))).into_response();
    }

    if video_url.is_empty() {
        return Html(templates::new_video_page(Some("Video URL is required."))).into_response();
    }

    let valid_categories = [
        "breathing",
        "meditation",
        "nutrition",
        "exercise",
        "mental-health",
        "general",
    ];
    if !valid_categories.contains(&category.as_str()) {
        return Html(templates::new_video_page(Some(
            "Please select a valid category.",
        )))
        .into_response();
    }

    match state
        .user_store
        .create_video(
            &user.id,
            title,
            form.description.trim().to_string(),
            video_url,
            form.thumbnail_url.trim().to_string(),
            category,
        )
        .await
    {
        Ok(id) => Redirect::to(&format!("/videos/{id}")).into_response(),
        Err(e) => Html(templates::new_video_page(Some(&e))).into_response(),
    }
}

pub async fn show_video(
    session: Session,
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Response {
    if let Err(r) = require_user(&session, &state).await {
        return r;
    }

    match state.user_store.get_video_by_id(&id).await {
        Some(video) => Html(templates::video_player_page(&video)).into_response(),
        None => Redirect::to("/videos").into_response(),
    }
}
