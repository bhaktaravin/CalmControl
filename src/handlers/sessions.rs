use axum::{
    extract::{Form, State},
    response::{Html, IntoResponse, Redirect, Response},
};
use serde::Deserialize;
use std::sync::Arc;
use tower_sessions::Session;

use crate::{handlers::auth::SESSION_USER_KEY, state::AppState, templates};

// ── Forms ──────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct JournalForm {
    pub mood: i64,
    pub note: String,
}

// ── Breathing ──────────────────────────────────────────────────────────────────

pub async fn show_breathe(session: Session, State(state): State<Arc<AppState>>) -> Response {
    let user_email = match session.get::<String>(SESSION_USER_KEY).await.ok().flatten() {
        Some(e) => e,
        None => return Redirect::to("/login").into_response(),
    };

    if state.user_store.find_by_email(&user_email).await.is_none() {
        let _ = session.flush().await;
        return Redirect::to("/login").into_response();
    }

    Html(templates::breathe_page()).into_response()
}

pub async fn complete_breathe(session: Session, State(state): State<Arc<AppState>>) -> Response {
    let user_email = match session.get::<String>(SESSION_USER_KEY).await.ok().flatten() {
        Some(e) => e,
        None => return Redirect::to("/login").into_response(),
    };

    let user = match state.user_store.find_by_email(&user_email).await {
        Some(u) => u,
        None => {
            let _ = session.flush().await;
            return Redirect::to("/login").into_response();
        }
    };

    let _ = state.user_store.log_session(&user.id, "breathing", 5).await;
    Redirect::to("/dashboard?completed=breathing").into_response()
}

// ── Meditation ─────────────────────────────────────────────────────────────────

pub async fn show_meditate(session: Session, State(state): State<Arc<AppState>>) -> Response {
    let user_email = match session.get::<String>(SESSION_USER_KEY).await.ok().flatten() {
        Some(e) => e,
        None => return Redirect::to("/login").into_response(),
    };

    if state.user_store.find_by_email(&user_email).await.is_none() {
        let _ = session.flush().await;
        return Redirect::to("/login").into_response();
    }

    Html(templates::meditate_page()).into_response()
}

pub async fn complete_meditate(session: Session, State(state): State<Arc<AppState>>) -> Response {
    let user_email = match session.get::<String>(SESSION_USER_KEY).await.ok().flatten() {
        Some(e) => e,
        None => return Redirect::to("/login").into_response(),
    };

    let user = match state.user_store.find_by_email(&user_email).await {
        Some(u) => u,
        None => {
            let _ = session.flush().await;
            return Redirect::to("/login").into_response();
        }
    };

    let _ = state
        .user_store
        .log_session(&user.id, "meditation", 10)
        .await;
    Redirect::to("/dashboard?completed=meditation").into_response()
}

// ── Journal ────────────────────────────────────────────────────────────────────

pub async fn show_journal(session: Session, State(state): State<Arc<AppState>>) -> Response {
    let user_email = match session.get::<String>(SESSION_USER_KEY).await.ok().flatten() {
        Some(e) => e,
        None => return Redirect::to("/login").into_response(),
    };

    if state.user_store.find_by_email(&user_email).await.is_none() {
        let _ = session.flush().await;
        return Redirect::to("/login").into_response();
    }

    Html(templates::journal_page(None)).into_response()
}

pub async fn submit_journal(
    session: Session,
    State(state): State<Arc<AppState>>,
    Form(form): Form<JournalForm>,
) -> Response {
    let user_email = match session.get::<String>(SESSION_USER_KEY).await.ok().flatten() {
        Some(e) => e,
        None => return Redirect::to("/login").into_response(),
    };

    let user = match state.user_store.find_by_email(&user_email).await {
        Some(u) => u,
        None => {
            let _ = session.flush().await;
            return Redirect::to("/login").into_response();
        }
    };

    if !(1..=5).contains(&form.mood) {
        return Html(templates::journal_page(Some("Please select a mood."))).into_response();
    }

    let note = form.note.trim().to_string();

    match state
        .user_store
        .log_journal_entry(&user.id, form.mood, &note)
        .await
    {
        Ok(_) => Redirect::to("/dashboard?completed=journal").into_response(),
        Err(e) => Html(templates::journal_page(Some(&e))).into_response(),
    }
}
