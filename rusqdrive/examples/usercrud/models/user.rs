use chrono::{DateTime, Utc};
use rusqdrive::prelude::*;

#[tablename = "users"]
#[derive(Debug, Rusqdrive)]
pub struct User {
    id: Option<i32>,
    #[rusqdrive(not_null = false, unique = false)]
    name: Option<String>,
    #[rusqdrive(not_null = false, unique = false)]
    age: u8,
    #[rusqdrive(not_null = false, unique = false)]
    alive: bool,
    #[rusqdrive(not_null = false, unique = false)]
    created_at: DateTime<Utc>,
    #[rusqdrive(not_null = false, unique = false)]
    updated_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn new(
        id: Option<i32>,
        name: Option<String>,
        age: u8,
        alive: bool,
        created_at: DateTime<Utc>,
        updated_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id,
            name,
            age,
            alive,
            created_at,
            updated_at,
        }
    }
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: None,
            name: None,
            age: 0,
            alive: false,
            created_at: Utc::now(),
            updated_at: None,
        }
    }
}
