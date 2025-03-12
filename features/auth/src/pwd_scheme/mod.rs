pub mod error;
mod scheme_01_argon2id;

use error::{Result, SchemeError};
use scheme_01_argon2id::Scheme01Argon2id;

use crate::password::ContentToHash;

pub const DEFAULT_SCHEME: &str = "01";

pub trait Scheme {
    fn hash(&self, to_hash: &ContentToHash) -> Result<String>;
    fn validate(&self, passwd: &str, passwd_ref: &str) -> Result<()>;
}

// #[derive(Debug)]
// pub enum PasswordScheme {
//     Argon2, // current default
// }

// impl std::fmt::Display for PasswordScheme {
//     fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             PasswordScheme::Argon2 => write!(fmt, "01"),
//         }
//     }
// }

#[derive(Debug, PartialEq, Eq)]
pub enum SchemeStatus {
    Ok,
    Outdated,
}

pub fn get_scheme(scheme_name: &str) -> Result<Box<dyn Scheme>> {
    match scheme_name {
        "01" => Ok(Box::new(Scheme01Argon2id)),
        _ => Err(SchemeError::SchemeNotFound(scheme_name.to_string())),
    }
}
