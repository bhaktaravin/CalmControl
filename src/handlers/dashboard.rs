use axum::{
    extract::State,
    response::{Html, IntoResponse, Redirect, Response},
};
use std::sync::Arc;
use tower_sessions::Session;

use crate::{handlers::auth::SESSION_USER_KEY, state::AppState, templates};

pub async fn show_dashboard(session: Session, State(state): State<Arc<AppState>>) -> Response {
    let user_email = match session.get::<String>(SESSION_USER_KEY).await.ok().flatten() {
        Some(email) => email,
        None => return Redirect::to("/login").into_response(),
    };

    let user = match state.user_store.find_by_email(&user_email).await {
        Some(u) => u,
        None => {
            let _ = session.flush().await;
            return Redirect::to("/login").into_response();
        }
    };

    let stats = state.user_store.get_stats(&user.id).await;
    let weekly = state.user_store.get_weekly_minutes(&user.id).await;

    Html(templates::dashboard_page(&user, &stats, &weekly)).into_response()
}
