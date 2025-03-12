use crate::pwd_scheme::error::SchemeError;

pub type Result<T> = std::result::Result<T, AuthError>;

#[derive(Debug)]
pub enum AuthError {
    JwtError(String),
    Unauthorized,

    Scheme(SchemeError),
}

impl From<SchemeError> for AuthError {
    fn from(value: SchemeError) -> Self {
        Self::Scheme(value)
    }
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            AuthError::JwtError(e) => write!(fmt, "JWT Error: {e}"),
            AuthError::Unauthorized => write!(fmt, "Unauthorized access"),
            AuthError::Scheme(e) => write!(fmt, "Scheme error: {e}"),
        }
    }
}

impl std::error::Error for AuthError {}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_auth_error_rendering() {
        assert_eq!(
            "Scheme error: Scheme not found: UnknownScheme",
            AuthError::Scheme(SchemeError::SchemeNotFound("UnknownScheme".to_string())).to_string()
        );
        assert_eq!(
            "JWT Error: Error Message",
            AuthError::JwtError("Error Message".to_string()).to_string()
        );
        assert_eq!("Unauthorized access", AuthError::Unauthorized.to_string());
    }
}
