use bcrypt::{DEFAULT_COST, hash, verify};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password_hash: String,
}

impl User {
    pub fn new(id: String, name: String, email: String, password: String) -> Result<Self, String> {
        let password_hash = hash(&password, DEFAULT_COST).map_err(|e| e.to_string())?;
        Ok(User {
            id,
            name,
            email,
            password_hash,
        })
    }

    pub fn verify_password(&self, password: &str) -> bool {
        verify(password, &self.password_hash).unwrap_or(false)
    }
}
