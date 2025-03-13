use async_trait::async_trait;

use super::models::User;

pub mod error;
pub mod in_mem_user_repo;

use error::Result;

#[async_trait]
pub trait UserRepositoryTrait: Send + Sync + 'static {
    async fn create_user(&self, user: User) -> Result<User>;
    async fn find_by_id(&self, id: &str) -> Result<Option<User>>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>>;
    async fn update_user(&self, user: &User) -> Result<User>;
}
