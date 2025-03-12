pub type Result<T> = std::result::Result<T, SchemeError>;

#[derive(Debug)]
pub enum SchemeError {
    Key,
    Salt,
    Hash,
    PasswordValidate,
    PasswordSchemeParse,
    SchemeNotFound(String),
}

impl std::fmt::Display for SchemeError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            SchemeError::Key
            | SchemeError::Salt
            | SchemeError::Hash
            | SchemeError::PasswordValidate
            | SchemeError::PasswordSchemeParse => {
                write!(fmt, "{self:?}")
            }
            SchemeError::SchemeNotFound(e) => write!(fmt, "Scheme not found: {e}"),
        }
    }
}

impl std::error::Error for SchemeError {}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_scheme_error_rendering() {
        assert_eq!("Key", SchemeError::Key.to_string());
        assert_eq!("Salt", SchemeError::Salt.to_string());
        assert_eq!("Hash", SchemeError::Hash.to_string());
        assert_eq!(
            "PasswordValidate",
            SchemeError::PasswordValidate.to_string()
        );
        assert_eq!(
            "PasswordSchemeParse",
            SchemeError::PasswordSchemeParse.to_string()
        );
        assert_eq!(
            "Scheme not found: Error Message",
            SchemeError::SchemeNotFound("Error Message".to_string()).to_string()
        );
    }
}
