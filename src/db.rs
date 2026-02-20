use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use std::str::FromStr;

pub async fn create_pool(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    let options = SqliteConnectOptions::from_str(database_url)?.create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    init_schema(&pool).await?;

    Ok(pool)
}

async fn init_schema(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id            TEXT PRIMARY KEY,
            name          TEXT NOT NULL,
            email         TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            created_at    TEXT NOT NULL DEFAULT (datetime('now'))
        )",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS mindful_sessions (
            id           TEXT PRIMARY KEY,
            user_id      TEXT NOT NULL,
            session_type TEXT NOT NULL,
            duration_min INTEGER NOT NULL DEFAULT 0,
            completed_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (user_id) REFERENCES users(id)
        )",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS journal_entries (
            id         TEXT PRIMARY KEY,
            user_id    TEXT NOT NULL,
            mood       INTEGER NOT NULL CHECK(mood BETWEEN 1 AND 5),
            note       TEXT NOT NULL DEFAULT '',
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (user_id) REFERENCES users(id)
        )",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS videos (
            id            TEXT PRIMARY KEY,
            user_id       TEXT NOT NULL,
            title         TEXT NOT NULL,
            description   TEXT NOT NULL DEFAULT '',
            video_url     TEXT NOT NULL,
            thumbnail_url TEXT NOT NULL DEFAULT '',
            category      TEXT NOT NULL DEFAULT 'general',
            created_at    TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (user_id) REFERENCES users(id)
        )",
    )
    .execute(pool)
    .await?;

    Ok(())
}
