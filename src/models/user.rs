use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[allow(dead_code)]
impl User {
    pub fn new(id: i32, name: String, email: String) -> Self {
        User { id, name, email }
    }
} 