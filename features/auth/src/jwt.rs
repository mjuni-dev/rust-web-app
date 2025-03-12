use jsonwebtoken::{
    DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode, errors::ErrorKind,
};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

use crate::error::{AuthError, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String, // Subject (user id)
    pub exp: i64,    // expiration time
    pub iat: i64,    // issued at
    pub jti: String, // jwt id
}

pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    expiration: Duration,
}

impl JwtService {
    pub fn new(secret: &[u8], expiration_hours: i64) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret),
            decoding_key: DecodingKey::from_secret(secret),
            expiration: Duration::hours(expiration_hours),
        }
    }

    pub fn generate_token(&self, user_id: &str) -> Result<String> {
        let now = OffsetDateTime::now_utc();
        let expiry = now + self.expiration;

        let claims = JwtClaims {
            sub: user_id.to_string(),
            exp: expiry.unix_timestamp(),
            iat: now.unix_timestamp(),
            jti: Uuid::new_v4().to_string(),
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AuthError::JwtError(e.to_string()))
    }

    pub fn validate_token(&self, token: &str) -> Result<JwtClaims> {
        let token_data: TokenData<JwtClaims> =
            decode(token, &self.decoding_key, &Validation::default()).map_err(|e| {
                match e.kind() {
                    ErrorKind::ExpiredSignature => AuthError::Unauthorized,
                    _ => AuthError::JwtError(e.to_string()),
                }
            })?;

        Ok(token_data.claims)
    }
}

#[cfg(test)]
mod tests {
    use crate::{config::auth_config, error::AuthError};

    use super::JwtService;
    use std::time::SystemTime;

    const TEST_SECRET: &[u8] = b"test_jwt_secret";
    const TEST_USER_ID: &str = "user-123";

    fn create_test_jwt_service() -> JwtService {
        let cfg = &auth_config();
        JwtService::new(cfg.token_key.as_bytes(), cfg.token_duration_hour)
    }

    #[test]
    fn test_generate_token() {
        let jwt_service = create_test_jwt_service();
        let token_result = jwt_service.generate_token(TEST_USER_ID);

        assert!(token_result.is_ok(), "Token should be generated");

        let token = token_result.unwrap();
        assert!(!token.is_empty(), "Token should not be empty");

        assert_eq!(token.matches('.').count(), 2, "Token should have 2 dots");
    }

    #[test]
    fn test_validate_valid_token() {
        let jwt_service = create_test_jwt_service();
        let token = jwt_service.generate_token(TEST_USER_ID).unwrap();

        let validation_result = jwt_service.validate_token(&token);

        assert!(
            validation_result.is_ok(),
            "Token should be successfully validated"
        );

        let claims = validation_result.unwrap();
        assert_eq!(
            claims.sub, TEST_USER_ID,
            "Subject claim should match the user id"
        );
    }

    #[test]
    fn test_validate_expired_token() {
        let jwt_service = JwtService::new(TEST_SECRET, -1);
        let token = jwt_service.generate_token("diff_user-123").unwrap();

        let validation_result = jwt_service.validate_token(&token);

        assert!(
            validation_result.is_err(),
            "Expired token should fail validation"
        );
        assert!(
            matches!(validation_result.unwrap_err(), AuthError::Unauthorized),
            "Error should be Unauthorized for expired token"
        );
    }

    #[test]
    fn test_validate_invalid_token() {
        let jwt_service = create_test_jwt_service();
        let invalid_token = "invalid.jwt.token";

        let validation_result = jwt_service.validate_token(invalid_token);

        assert!(
            validation_result.is_err(),
            "Invalid token should fail validation"
        );
        assert!(
            matches!(validation_result.unwrap_err(), AuthError::JwtError(_)),
            "Error should be JwtError for invalid token format"
        );
    }

    #[test]
    fn test_validate_token_from_different_secret() {
        let jwt_service1 = JwtService::new(b"secret1", 24);
        let jwt_service2 = JwtService::new(b"secret2", 24);

        let token = jwt_service1.generate_token(TEST_USER_ID).unwrap();
        let validation_result = jwt_service2.validate_token(&token);

        assert!(
            validation_result.is_err(),
            "Token signed with different secret should fail validation"
        );
    }

    #[test]
    fn test_token_contains_expected_claims() {
        let jwt_service = create_test_jwt_service();
        let token = jwt_service.generate_token(TEST_USER_ID).unwrap();

        let claims = jwt_service.validate_token(&token).unwrap();

        // Verify subject
        assert_eq!(claims.sub, TEST_USER_ID);

        // Verify expiration is in the future
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        assert!(claims.exp > now, "Expiration time should be in the future");

        // Verify issued at is in the past or present
        assert!(
            claims.iat <= now,
            "Issued at time should be in the past or present"
        );

        // Verify JWT ID is present
        assert!(!claims.jti.is_empty(), "JWT ID should not be empty");
    }
}
