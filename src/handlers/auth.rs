use axum::{
    extract::{Form, State},
    response::{Html, IntoResponse, Redirect, Response},
};
use serde::Deserialize;
use std::sync::Arc;
use tower_sessions::Session;

use crate::{state::AppState, templates};

pub const SESSION_USER_KEY: &str = "user_email";

#[derive(Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterForm {
    pub name: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

pub async fn show_login_page(session: Session) -> Response {
    if let Ok(Some(_)) = session.get::<String>(SESSION_USER_KEY).await {
        return Redirect::to("/dashboard").into_response();
    }
    Html(templates::login_page(None)).into_response()
}

pub async fn process_login(
    session: Session,
    State(state): State<Arc<AppState>>,
    Form(form): Form<LoginForm>,
) -> Response {
    let email = form.email.trim().to_lowercase();

    match state.user_store.find_by_email(&email).await {
        Some(user) if user.verify_password(&form.password) => {
            let _ = session.insert(SESSION_USER_KEY, user.email.clone()).await;
            Redirect::to("/dashboard").into_response()
        }
        _ => Html(templates::login_page(Some(
            "Invalid email or password. Please try again.",
        )))
        .into_response(),
    }
}

pub async fn logout(session: Session) -> Response {
    let _ = session.flush().await;
    Redirect::to("/login").into_response()
}

pub async fn show_register_page(session: Session) -> Response {
    if let Ok(Some(_)) = session.get::<String>(SESSION_USER_KEY).await {
        return Redirect::to("/dashboard").into_response();
    }
    Html(templates::register_page(None)).into_response()
}

pub async fn process_register(
    session: Session,
    State(state): State<Arc<AppState>>,
    Form(form): Form<RegisterForm>,
) -> Response {
    let name = form.name.trim().to_string();
    let email = form.email.trim().to_lowercase();

    if name.is_empty() || email.is_empty() || form.password.is_empty() {
        return Html(templates::register_page(Some("All fields are required."))).into_response();
    }

    if form.password != form.confirm_password {
        return Html(templates::register_page(Some("Passwords do not match."))).into_response();
    }

    if form.password.len() < 8 {
        return Html(templates::register_page(Some(
            "Password must be at least 8 characters long.",
        )))
        .into_response();
    }

    match state
        .user_store
        .create_user(name, email.clone(), form.password)
        .await
    {
        Ok(_) => {
            let _ = session.insert(SESSION_USER_KEY, email).await;
            Redirect::to("/dashboard").into_response()
        }
        Err(e) => Html(templates::register_page(Some(&e))).into_response(),
    }
}
