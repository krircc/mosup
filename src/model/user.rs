use chrono::{Utc, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct User {
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: Option<DateTime<Utc>>,
    pub avatar: Option<String>,
}

impl User {
    pub fn new() -> User {
        User {
            email: "".to_owned(),
            username: "".to_owned(),
            password: "".to_owned(),
            created_at: Some(Utc::now()),
            avatar: Some("".to_string()),
        }
    }
}