use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Note {
    pub fn new(id: i32, title: String, content: String) -> Self {
        let now = Utc::now();
        Note {
            id,
            title,
            content,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update(&mut self, title: String, content: String) {
        self.title = title;
        self.content = content;
        self.updated_at = Utc::now();
    }
}