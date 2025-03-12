use std::sync::OnceLock;

use argon2::password_hash::SaltString;
use argon2::{
    Algorithm, Argon2, Params, PasswordHash, PasswordHasher as _, PasswordVerifier, Version,
};

use crate::pwd_scheme::error::SchemeError;
use crate::{config::auth_config, password::ContentToHash};

use super::Scheme;
use super::error::Result;

pub struct Scheme01Argon2id;

impl Scheme for Scheme01Argon2id {
    fn hash(&self, to_hash: &ContentToHash) -> Result<String> {
        let argon2 = get_argon2();

        let salt_b64 =
            SaltString::encode_b64(to_hash.salt.as_bytes()).map_err(|_| SchemeError::Salt)?;

        let passwd = argon2
            .hash_password(to_hash.content.as_bytes(), &salt_b64)
            .map_err(|_| SchemeError::Hash)?
            .to_string();

        // sample hash: $argon2id$v=19$m=16,t=2,p=1$salt$hash
        Ok(passwd)
    }

    fn validate(&self, passwd: &str, passwd_ref: &str) -> Result<()> {
        let argon2 = get_argon2();

        let parsed_hash = PasswordHash::new(passwd_ref).map_err(|_| SchemeError::Hash)?;

        argon2
            .verify_password(passwd.as_bytes(), &parsed_hash)
            .map_err(|_| SchemeError::PasswordValidate)
    }
}

fn get_argon2() -> &'static Argon2<'static> {
    static INSTANCE: OnceLock<Argon2<'static>> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        let key = &auth_config().pwd_key;
        Argon2::new_with_secret(
            key.as_bytes(),
            Algorithm::Argon2id,
            Version::V0x13,
            Params::default(),
        )
        .unwrap()
    })
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::pwd_scheme::get_scheme;

    use super::*;

    const TEST_PASSWORD: &str = "TestPassword123$";
    const TEST_INCORRECT_PASSWORD: &str = "TestIncorrectPassword123$";

    #[test]
    fn test_hash_password() {
        let scheme = get_scheme("01").unwrap();
        let content = ContentToHash {
            content: TEST_PASSWORD.to_string(),
            salt: Uuid::new_v4(),
        };

        let hash_result = scheme.hash(&content);
        assert!(hash_result.is_ok(), "Password hash should succeed");

        let hash = hash_result.unwrap();
        assert!(!hash.is_empty(), "Hash should not be empty");
        assert!(
            hash.starts_with("$argon2id$"),
            "Hash should start with the scheme prefix"
        );
    }

    #[test]
    fn test_validate_with_correct_password() {
        let scheme = get_scheme("01").unwrap();
        let content = ContentToHash {
            content: TEST_PASSWORD.to_string(),
            salt: Uuid::new_v4(),
        };

        let hash = scheme.hash(&content).unwrap();
        let validate_result = scheme.validate(TEST_PASSWORD, &hash);
        assert!(
            validate_result.is_ok(),
            "Password validation should succeed"
        );
    }

    #[test]
    fn test_validate_with_incorrect_password() {
        let scheme = get_scheme("01").unwrap();
        let content = ContentToHash {
            content: TEST_PASSWORD.to_string(),
            salt: Uuid::new_v4(),
        };

        let hash = scheme.hash(&content).unwrap();
        let validate_result = scheme.validate(TEST_INCORRECT_PASSWORD, &hash);
        assert!(!validate_result.is_ok(), "Incorrect Password should fail");
    }
}
