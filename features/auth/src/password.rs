use std::str::FromStr;
use uuid::Uuid;

use crate::{
    error::{AuthError, Result},
    pwd_scheme::{DEFAULT_SCHEME, SchemeStatus, error::SchemeError, get_scheme},
};

pub struct ContentToHash {
    pub content: String,
    pub salt: Uuid,
}

pub struct PasswordParts {
    scheme_name: String,
    hash: String,
}

const SCHEME_DELIMETER: char = '#';

impl FromStr for PasswordParts {
    type Err = AuthError;

    fn from_str(pwd_with_scheme: &str) -> Result<Self> {
        let parts: Vec<&str> = pwd_with_scheme.splitn(2, SCHEME_DELIMETER).collect();
        if parts.len() != 2 {
            return Err(AuthError::from(SchemeError::PasswordSchemeParse));
        }

        Ok(PasswordParts {
            scheme_name: parts[0].to_string(),
            hash: parts[1].to_string(),
        })
    }
}

pub fn hash_password(to_hash: &ContentToHash) -> Result<String> {
    hash_for_scheme(DEFAULT_SCHEME, to_hash)
}

fn hash_for_scheme(scheme_name: &str, to_hash: &ContentToHash) -> Result<String> {
    let scheme = get_scheme(scheme_name)?;
    let pwd_hash = scheme.hash(to_hash)?;

    Ok(format!("{scheme_name}{SCHEME_DELIMETER}{pwd_hash}"))
}

pub fn verify_password(passwd: &str, passwd_ref: &str) -> Result<SchemeStatus> {
    let PasswordParts { scheme_name, hash } = passwd_ref.parse()?;

    verify_with_scheme(&scheme_name, passwd, &hash)?;

    if scheme_name == DEFAULT_SCHEME {
        Ok(SchemeStatus::Ok)
    } else {
        Ok(SchemeStatus::Outdated)
    }
}

fn verify_with_scheme(scheme_name: &str, passwd: &str, pwd_ref: &str) -> Result<()> {
    get_scheme(scheme_name)?.validate(passwd, pwd_ref)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_PASSWORD: &str = "TestPassword123$";

    // this hash was generated with TEST_PASSWORD
    const TEST_HASH_OK: &str = "01#$argon2id$v=19$m=19456,t=2,p=1$fNNcBL/FRUi8WfpMK6bpzA$bf4xoz3GFXZC5Hx0I4yQD7xJufC7KaoOFf6ZV6+YA4A";
    const TEST_UNKNOWN_SCHEME_HASH: &str = "unknown_scheme#$argon2id$v=19$m=19456,t=2,p=1$fNNcBL/FRUi8WfpMK6bpzA$bf4xoz3GFXZC5Hx0I4yQD7xJufC7KaoOFf6ZV6+YA4A";
    const TEST_NO_SCHEME_HASH: &str = "$argon2id$v=19$m=19456,t=2,p=1$fNNcBL/FRUi8WfpMK6bpzA$bf4xoz3GFXZC5Hx0I4yQD7xJufC7KaoOFf6ZV6+YA4A";

    #[test]
    fn test_multi_scheme_ok() {
        let test_to_hash = ContentToHash {
            content: TEST_PASSWORD.to_string(),
            salt: Uuid::new_v4(),
        };

        let hash_result = hash_password(&test_to_hash);
        assert!(hash_result.is_ok(), "Password hash should succeed");

        let hash = hash_result.unwrap();
        assert!(!hash.is_empty(), "Hash should not be empty");

        let validate_result = verify_password(&TEST_PASSWORD, &hash);
        assert!(
            validate_result.is_ok(),
            "Password validation should succeed"
        );
    }

    #[test]
    fn test_scheme_status_ok() {
        let schema_status = verify_password(&TEST_PASSWORD, &TEST_HASH_OK).unwrap();
        assert_eq!(
            schema_status,
            SchemeStatus::Ok,
            "Current scheme does not need rehashing"
        );
    }

    #[test]
    fn test_scheme_status_not_found() {
        let scheme_status_result = verify_password(&TEST_PASSWORD, &TEST_UNKNOWN_SCHEME_HASH);
        assert!(
            scheme_status_result.is_err(),
            "Should not validate unknown scheme"
        );
    }

    #[test]
    fn test_no_scheme_in_hash() {
        let scheme_status_result = verify_password(&TEST_PASSWORD, &TEST_NO_SCHEME_HASH);
        assert!(
            scheme_status_result.is_err(),
            "Password should fail if missing scheme"
        );
    }
}
