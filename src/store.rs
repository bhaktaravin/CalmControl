use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::user::User;

#[derive(Clone, Debug)]
pub struct UserStore {
    pool: SqlitePool,
}

impl UserStore {
    pub fn new(pool: SqlitePool) -> Self {
        UserStore { pool }
    }

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
}
