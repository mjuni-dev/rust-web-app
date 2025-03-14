use async_trait::async_trait;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use time::OffsetDateTime;

use super::error::Result;
use super::{UserRepositoryTrait, error::RepoError};

use crate::models::User;

pub struct InMemoryUserRepository {
    users: Arc<RwLock<HashMap<String, User>>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl UserRepositoryTrait for InMemoryUserRepository {
    async fn create_user(&self, user: User) -> Result<User> {
        let mut users = self
            .users
            .write()
            .map_err(|_| return RepoError::CreateUser)?;

        let user_clone = user.clone();
        users.insert(user.id.clone(), user);
        Ok(user_clone)
    }
    async fn find_by_id(&self, id: &str) -> Result<Option<User>> {
        let users = self.users.read().map_err(|_| {
            return RepoError::DataReadError;
        })?;

        Ok(users.get(id).cloned())
    }
    async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let users = self.users.read().map_err(|_| {
            return RepoError::DataReadError;
        })?;

        for user in users.values() {
            if user.email == email {
                return Ok(Some(user.clone()));
            }
        }

        Ok(None)
    }
    async fn update_user(&self, user: &User) -> Result<User> {
        let mut users = self
            .users
            .write()
            .map_err(|_| return RepoError::UpdateUser)?;

        if !users.contains_key(&user.id) {
            return Err(RepoError::UpdateUser);
        }

        let updated_user = User {
            updated_at: OffsetDateTime::now_utc().unix_timestamp(),
            ..user.clone()
        };

        users.insert(user.id.clone(), updated_user.clone());
        Ok(updated_user)
    }
}
