use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Video {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub description: String,
    pub video_url: String,
    pub thumbnail_url: String,
    pub category: String,
    pub created_at: String,
}

/// Video joined with the uploader's display name.
#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct VideoWithUploader {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub description: String,
    pub video_url: String,
    pub thumbnail_url: String,
    pub category: String,
    pub created_at: String,
    pub uploader_name: String,
}

/// All valid video categories.
pub const CATEGORIES: &[(&str, &str)] = &[
    ("breathing", "🌬️  Breathing"),
    ("meditation", "🧘  Meditation"),
    ("nutrition", "🥗  Nutrition"),
    ("exercise", "💪  Exercise"),
    ("mental-health", "🧠  Mental Health"),
    ("general", "🌿  General"),
];

/// Return the emoji + label for a category slug, falling back gracefully.
pub fn category_label(slug: &str) -> &'static str {
    CATEGORIES
        .iter()
        .find(|(s, _)| *s == slug)
        .map(|(_, label)| *label)
        .unwrap_or("🌿  General")
}
