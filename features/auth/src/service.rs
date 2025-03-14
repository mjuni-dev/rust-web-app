use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::error::{AuthError, Result};
use crate::jwt::JwtService;
use crate::models::{Credentials, RegisterUser, User};
use crate::password::{self, hash_password, verify_password};
use crate::pwd_scheme::SchemeStatus;
use crate::repository::UserRepositoryTrait;

#[async_trait]
pub trait AuthServiceTrait: Send + Sync + 'static {
    async fn register(&self, user_data: RegisterUser) -> Result<User>;
    async fn signin(&self, creds: Credentials) -> Result<String>;
    async fn validate_token(&self, token: &str) -> Result<User>;
}

pub struct AuthService<R: UserRepositoryTrait> {
    user_repo: Arc<R>,
    jwt_service: Arc<JwtService>,
}

impl<R: UserRepositoryTrait> AuthService<R> {
    pub fn new(user_repo: Arc<R>, jwt_service: Arc<JwtService>) -> Self {
        Self {
            user_repo,
            jwt_service,
        }
    }
}

#[async_trait]
impl<R: UserRepositoryTrait> AuthServiceTrait for AuthService<R> {
    async fn register(&self, user_data: RegisterUser) -> Result<User> {
        validate_email(&user_data.email)?;

        // TODO: implement more robust password validation
        if user_data.password.len() < 8 {
            return Err(AuthError::PasswordValidation(
                "Password must be at least 8 characters".to_string(),
            ));
        }

        if let Ok(Some(_)) = self.user_repo.find_by_email(&user_data.email).await {
            return Err(AuthError::UserExists);
        }

        let password_hash = password::hash_password(&password::ContentToHash {
            content: user_data.password,
            salt: Uuid::new_v4(),
        })?;

        let user = User::new(user_data.email, password_hash, user_data.name);
        let user = self.user_repo.create_user(user).await?;

        Ok(user)
    }

    async fn signin(&self, creds: Credentials) -> Result<String> {
        let mut user = match self.user_repo.find_by_email(&creds.email).await? {
            Some(user) => user,
            None => return Err(AuthError::InvalidCredentials),
        };

        match verify_password(&creds.password, &user.password)? {
            SchemeStatus::Ok => {}
            SchemeStatus::Outdated => {
                let new_hash = hash_password(&password::ContentToHash {
                    content: creds.password,
                    salt: Uuid::new_v4(),
                })?;

                user.password = new_hash;

                self.user_repo.update_user(&user).await?;
            }
        }

        self.jwt_service.generate_token(&user.id)
    }

    async fn validate_token(&self, token: &str) -> Result<User> {
        let claims = self.jwt_service.validate_token(token)?;

        let user = match self.user_repo.find_by_id(&claims.sub).await? {
            Some(user) => user,
            None => return Err(AuthError::UserNotFound),
        };

        Ok(user)
    }
}

// Simple email validation
fn validate_email(email: &str) -> Result<bool> {
    match regex::Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    ) {
        Ok(r) => Ok(r.is_match(email)),
        Err(_) => Err(AuthError::EmailValidation),
    }
}

#[cfg(test)]
mod tests {
    use crate::{InMemoryUserRepository, password::ContentToHash};

    use super::*;
    // use auth::{
    //     AuthService, AuthServiceImpl, Credentials, JwtService, RegisterUser, User,
    //     password::{hash_password, needs_rehash, verify_password},
    //     repository::memory::InMemoryUserRepository,
    // };
    use std::sync::Arc;

    #[tokio::test]
    async fn test_auth_service_end_to_end() {
        // Setup dependencies
        let user_repository = Arc::new(InMemoryUserRepository::new());
        let jwt_service = Arc::new(JwtService::new(b"test_secret", 24));

        // Create auth service
        let auth_service = AuthService::new(user_repository, jwt_service.clone());

        // Test data
        let register_data = RegisterUser {
            email: "test@example.com".to_string(),
            password: "Password123!".to_string(),
            name: "Test User".to_string(),
        };

        // Test registration
        let registered_user = auth_service.register(register_data.clone()).await.unwrap();
        assert_eq!(registered_user.email, "test@example.com");
        assert_eq!(registered_user.name, "Test User".to_string());

        // Test login
        let credentials = Credentials {
            email: "test@example.com".to_string(),
            password: "Password123!".to_string(),
        };

        let token = auth_service.signin(credentials).await.unwrap();
        assert!(!token.is_empty());

        // Test token validation
        let user = auth_service.validate_token(&token).await.unwrap();
        assert_eq!(user.email, "test@example.com");

        // Test invalid login
        let invalid_credentials = Credentials {
            email: "test@example.com".to_string(),
            password: "WrongPassword123!".to_string(),
        };

        let login_result = auth_service.signin(invalid_credentials).await;
        assert!(login_result.is_err());
    }

    #[tokio::test]
    async fn test_duplicate_user_registration() {
        // Setup dependencies
        let user_repository = Arc::new(InMemoryUserRepository::new());
        let jwt_service = Arc::new(JwtService::new(b"test_secret", 24));

        // Create auth service
        let auth_service = AuthService::new(user_repository, jwt_service.clone());

        // Test data
        let register_data = RegisterUser {
            email: "duplicate@example.com".to_string(),
            password: "Password123!".to_string(),
            name: "Test User".to_string(),
        };

        // First registration should succeed
        let registered_user = auth_service.register(register_data.clone()).await.unwrap();
        assert_eq!(registered_user.email, "duplicate@example.com");

        // Second registration with same email should fail
        let duplicate_result = auth_service.register(register_data).await;
        assert!(duplicate_result.is_err());
        match duplicate_result {
            Err(AuthError::UserExists) => (),
            _ => panic!("Expected UserAlreadyExists error"),
        }
    }
}
