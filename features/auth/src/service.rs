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
    jwt_service: JwtService,
}

impl<R: UserRepositoryTrait> AuthService<R> {
    pub fn new(user_repo: Arc<R>, jwt_service: JwtService) -> Self {
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
                });

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
