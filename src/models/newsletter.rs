use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct NewsletterSubscriber {
    pub id: String,
    pub email: String,
    pub name: String,
    pub unsubscribe_token: String,
    pub subscribed_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct NewsletterArticle {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub content_html: String,
    pub source_urls: String,
    pub published_at: String,
}
