use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
    pub name: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl User {
    pub fn new(email: String, password: String, name: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            email,
            password,
            name,
            created_at: OffsetDateTime::now_utc().unix_timestamp(),
            updated_at: OffsetDateTime::now_utc().unix_timestamp(),
        }
    }
}

#[derive(Debug)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

#[derive(Debug)]
pub struct RegisterUser {
    pub email: String,
    pub password: String,
    pub name: String,
}
