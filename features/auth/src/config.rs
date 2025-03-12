use std::sync::OnceLock;

use crate::Result;

pub fn auth_config() -> &'static AuthConfig {
    static INSTANCE: OnceLock<AuthConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        AuthConfig::load_from_env()
            .unwrap_or_else(|e| panic!("FATAL - WHILE LOADING CONFIG - Cause: {e:?}"))
    })
}

pub struct AuthConfig {
    pub pwd_key: &'static str,
    pub token_key: &'static str,
    pub token_duration_hour: i64,
}

impl AuthConfig {
    fn load_from_env() -> Result<AuthConfig> {
        // load from env
        Ok(AuthConfig {
            pwd_key: "1234",
            token_key: "5678",
            token_duration_hour: 24,
        })
    }
}
