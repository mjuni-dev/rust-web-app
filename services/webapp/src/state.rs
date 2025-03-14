use std::sync::Arc;

use crate::Result;

use auth::{AuthService, InMemoryUserRepository, JwtService};

pub struct AppState {
    user_repository: Arc<InMemoryUserRepository>,
    auth_service: Arc<AuthService<InMemoryUserRepository>>,
    // user_repository: Arc<UserRepositoryTrait>,
    // auth_service: Arc<AuthServiceTrait>,
}

impl AppState {
    pub async fn new() -> Result<Self> {
        let user_repository = Arc::new(InMemoryUserRepository::new());

        let jwt_service = Arc::new(JwtService::new("jwt_secret".as_bytes(), 24));

        let auth_service = Arc::new(AuthService::new(user_repository.clone(), jwt_service));

        Ok(Self {
            user_repository,
            auth_service,
        })
    }

    pub fn auth_service(&self) -> &Arc<AuthService<InMemoryUserRepository>> {
        &self.auth_service
    }
}
