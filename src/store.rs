use chrono::{Local, NaiveDate};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::{
    session::{DashboardStats, WeeklyMinutes},
    user::User,
    video::VideoWithUploader,
};

#[derive(Clone, Debug)]
pub struct UserStore {
    pool: SqlitePool,
}

impl UserStore {
    pub fn new(pool: SqlitePool) -> Self {
        UserStore { pool }
    }

    // ── Users ──────────────────────────────────────────────────────────────────

    pub async fn create_user(
        &self,
        name: String,
        email: String,
        password: String,
    ) -> Result<User, String> {
        let existing = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users WHERE email = ?")
            .bind(&email)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if existing > 0 {
            return Err("An account with this email already exists.".to_string());
        }

        let user = User::new(Uuid::new_v4().to_string(), name, email, password)?;

        sqlx::query("INSERT INTO users (id, name, email, password_hash) VALUES (?, ?, ?, ?)")
            .bind(&user.id)
            .bind(&user.name)
            .bind(&user.email)
            .bind(&user.password_hash)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(user)
    }

    pub async fn find_by_email(&self, email: &str) -> Option<User> {
        sqlx::query_as::<_, User>(
            "SELECT id, name, email, password_hash FROM users WHERE email = ?",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .ok()
        .flatten()
    }

    // ── Sessions ───────────────────────────────────────────────────────────────

    pub async fn log_session(
        &self,
        user_id: &str,
        session_type: &str,
        duration_min: i64,
    ) -> Result<(), String> {
        sqlx::query(
            "INSERT INTO mindful_sessions (id, user_id, session_type, duration_min)
             VALUES (?, ?, ?, ?)",
        )
        .bind(Uuid::new_v4().to_string())
        .bind(user_id)
        .bind(session_type)
        .bind(duration_min)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    // ── Journal ────────────────────────────────────────────────────────────────

    pub async fn log_journal_entry(
        &self,
        user_id: &str,
        mood: i64,
        note: &str,
    ) -> Result<(), String> {
        sqlx::query("INSERT INTO journal_entries (id, user_id, mood, note) VALUES (?, ?, ?, ?)")
            .bind(Uuid::new_v4().to_string())
            .bind(user_id)
            .bind(mood)
            .bind(note)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        // Also count a journal entry as a 5-minute mindful session
        self.log_session(user_id, "journal", 5).await
    }

    // ── Stats ──────────────────────────────────────────────────────────────────

    pub async fn get_stats(&self, user_id: &str) -> DashboardStats {
        let sessions_today = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM mindful_sessions
             WHERE user_id = ? AND date(completed_at) = date('now')",
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .unwrap_or(0);

        let total_minutes = sqlx::query_scalar::<_, i64>(
            "SELECT COALESCE(SUM(duration_min), 0) FROM mindful_sessions WHERE user_id = ?",
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .unwrap_or(0);

        // Fetch distinct session dates newest-first to calculate streak
        let dates = sqlx::query_scalar::<_, String>(
            "SELECT DISTINCT date(completed_at) FROM mindful_sessions
             WHERE user_id = ?
             ORDER BY 1 DESC",
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .unwrap_or_default();

        let streak = calculate_streak(&dates);

        DashboardStats {
            sessions_today,
            streak,
            total_minutes,
        }
    }

    pub async fn get_weekly_minutes(&self, user_id: &str) -> WeeklyMinutes {
        let rows = sqlx::query_as::<_, (String, i64)>(
            "SELECT date(completed_at) as d, COALESCE(SUM(duration_min), 0) as total_min
             FROM mindful_sessions
             WHERE user_id = ? AND date(completed_at) >= date('now', '-6 days')
             GROUP BY d
             ORDER BY d ASC",
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .unwrap_or_default();

        let mut result: WeeklyMinutes = [0; 7];
        let today = Local::now().date_naive();

        for (date_str, minutes) in rows {
            if let Ok(date) = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
                let days_ago = (today - date).num_days();
                if (0..7).contains(&days_ago) {
                    result[(6 - days_ago) as usize] = minutes;
                }
            }
        }

        result
    }

    // ── Videos ─────────────────────────────────────────────────────────────────

    pub async fn create_video(
        &self,
        user_id: &str,
        title: String,
        description: String,
        video_url: String,
        thumbnail_url: String,
        category: String,
    ) -> Result<String, String> {
        let id = Uuid::new_v4().to_string();

        sqlx::query(
            "INSERT INTO videos (id, user_id, title, description, video_url, thumbnail_url, category)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&id)
        .bind(user_id)
        .bind(&title)
        .bind(&description)
        .bind(&video_url)
        .bind(&thumbnail_url)
        .bind(&category)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(id)
    }

    pub async fn get_all_videos(&self) -> Vec<VideoWithUploader> {
        sqlx::query_as::<_, VideoWithUploader>(
            "SELECT v.id, v.user_id, v.title, v.description, v.video_url,
                    v.thumbnail_url, v.category, v.created_at,
                    u.name AS uploader_name
             FROM videos v
             JOIN users u ON v.user_id = u.id
             ORDER BY v.created_at DESC",
        )
        .fetch_all(&self.pool)
        .await
        .unwrap_or_default()
    }

    pub async fn get_video_by_id(&self, id: &str) -> Option<VideoWithUploader> {
        sqlx::query_as::<_, VideoWithUploader>(
            "SELECT v.id, v.user_id, v.title, v.description, v.video_url,
                    v.thumbnail_url, v.category, v.created_at,
                    u.name AS uploader_name
             FROM videos v
             JOIN users u ON v.user_id = u.id
             WHERE v.id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .ok()
        .flatten()
    }
}

// ── Helpers ────────────────────────────────────────────────────────────────────

/// Given a list of ISO date strings ordered newest-first, count how many
/// consecutive days ending today (or yesterday) have at least one session.
fn calculate_streak(dates: &[String]) -> i64 {
    let today = Local::now().date_naive();
    let mut streak = 0i64;
    let mut expected = today;

    for date_str in dates {
        if let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
            if date == expected {
                streak += 1;
                expected = expected.pred_opt().unwrap_or(expected);
            } else {
                break;
            }
        }
    }

    streak
}
