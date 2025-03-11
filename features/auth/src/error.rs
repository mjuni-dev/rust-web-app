pub type Result<T> = std::result::Result<T, AuthError>;

#[derive(Debug)]
pub enum AuthError {
    JwtError(String),
    Unauthorized,
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            AuthError::JwtError(e) => write!(fmt, "JWT Error: {e}"),
            AuthError::Unauthorized => write!(fmt, "Unauthorized access"),
        }
    }
}

impl std::error::Error for AuthError {}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_error_rendering() {
        assert_eq!(
            "JWT Error: Error Message",
            AuthError::JwtError("Error Message".to_string()).to_string()
        );
        assert_eq!("Unauthorized access", AuthError::Unauthorized.to_string());
    }
}
