use axum::{
    Form,
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse, Json, Redirect, Response},
};
use serde::{Deserialize, Serialize};
use std::{env, sync::Arc};
use tower_sessions::Session;

use crate::{state::AppState, templates};

// ── Forms & query params ───────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct SubscribeForm {
    pub email: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct UnsubscribeQuery {
    pub token: String,
}

// ── API request / response types ───────────────────────────────────────────────

#[derive(Deserialize)]
pub struct ArticlePayload {
    pub title: String,
    pub summary: String,
    pub content_html: String,
    pub source_urls: String,
}

#[derive(Serialize)]
struct SubscriberDto {
    email: String,
    name: String,
    unsubscribe_token: String,
}

#[derive(Serialize)]
struct ArticleCreatedDto {
    id: String,
    published_at: String,
}

// ── API key guard ──────────────────────────────────────────────────────────────

fn api_key_valid(headers: &HeaderMap) -> bool {
    let expected = env::var("NEWSLETTER_API_KEY").unwrap_or_default();
    if expected.is_empty() {
        return false;
    }

    // Accept: Authorization: Bearer <key>  OR  X-Api-Key: <key>
    let from_auth = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .map(|v| v == format!("Bearer {expected}"))
        .unwrap_or(false);

    let from_header = headers
        .get("x-api-key")
        .and_then(|v| v.to_str().ok())
        .map(|v| v == expected)
        .unwrap_or(false);

    from_auth || from_header
}

// ── Public: archive ────────────────────────────────────────────────────────────

pub async fn show_newsletter(State(state): State<Arc<AppState>>) -> Response {
    let articles = state.user_store.get_all_newsletter_articles().await;
    Html(templates::newsletter_page(&articles)).into_response()
}

// ── Public: single article ─────────────────────────────────────────────────────

pub async fn show_article(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> Response {
    match state.user_store.get_newsletter_article_by_id(&id).await {
        Some(article) => Html(templates::newsletter_article_page(&article)).into_response(),
        None => Redirect::to("/newsletter").into_response(),
    }
}

// ── Public: subscribe ──────────────────────────────────────────────────────────

pub async fn show_subscribe(_session: Session) -> Response {
    Html(templates::newsletter_subscribe_page(false, None)).into_response()
}

pub async fn process_subscribe(
    State(state): State<Arc<AppState>>,
    Form(form): Form<SubscribeForm>,
) -> Response {
    let email = form.email.trim().to_lowercase();
    let name = form.name.trim().to_string();

    if email.is_empty() || !email.contains('@') {
        return Html(templates::newsletter_subscribe_page(
            false,
            Some("Please enter a valid email address."),
        ))
        .into_response();
    }

    match state.user_store.subscribe(email, name).await {
        Ok(_) => Html(templates::newsletter_subscribe_page(true, None)).into_response(),
        Err(e) => Html(templates::newsletter_subscribe_page(false, Some(&e))).into_response(),
    }
}

// ── Public: unsubscribe ────────────────────────────────────────────────────────

pub async fn process_unsubscribe(
    State(state): State<Arc<AppState>>,
    Query(params): Query<UnsubscribeQuery>,
) -> Response {
    let token = params.token.trim().to_string();

    if token.is_empty() {
        return Html(templates::newsletter_unsubscribe_page(false)).into_response();
    }

    match state.user_store.unsubscribe_by_token(&token).await {
        Ok(_) => Html(templates::newsletter_unsubscribe_page(true)).into_response(),
        Err(_) => Html(templates::newsletter_unsubscribe_page(false)).into_response(),
    }
}

// ── n8n API: GET /api/newsletter/subscribers ───────────────────────────────────

pub async fn api_get_subscribers(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
) -> Response {
    if !api_key_valid(&headers) {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({ "error": "Unauthorized" })),
        )
            .into_response();
    }

    let subs = state.user_store.get_all_subscribers().await;
    let dto: Vec<SubscriberDto> = subs
        .into_iter()
        .map(|s| SubscriberDto {
            email: s.email,
            name: s.name,
            unsubscribe_token: s.unsubscribe_token,
        })
        .collect();

    Json(dto).into_response()
}

// ── n8n API: POST /api/newsletter/article ──────────────────────────────────────

pub async fn api_post_article(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ArticlePayload>,
) -> Response {
    if !api_key_valid(&headers) {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({ "error": "Unauthorized" })),
        )
            .into_response();
    }

    let title = payload.title.trim().to_string();
    let summary = payload.summary.trim().to_string();

    if title.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "title is required" })),
        )
            .into_response();
    }

    match state
        .user_store
        .create_newsletter_article(title, summary, payload.content_html, payload.source_urls)
        .await
    {
        Ok(article) => (
            StatusCode::CREATED,
            Json(ArticleCreatedDto {
                id: article.id,
                published_at: article.published_at,
            }),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e })),
        )
            .into_response(),
    }
}
