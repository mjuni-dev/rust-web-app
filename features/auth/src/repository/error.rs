use crate::AuthError;

pub type Result<T> = std::result::Result<T, RepoError>;

#[derive(Debug)]
pub enum RepoError {
    DataReadError,
    CreateUser,
    UpdateUser,
    UserNotFound,
}

impl std::fmt::Display for RepoError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for RepoError {}
