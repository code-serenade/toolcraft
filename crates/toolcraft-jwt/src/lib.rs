pub mod error;
mod jwt;
mod verify;

pub use jwt::{Claims, Jwt, JwtCfg, TokenPair};
pub use verify::{VerifyJwt, VerifyJwtCfg};

use crate::error::Error;
pub type Result<T> = std::result::Result<T, Error>;

pub trait AccessTokenVerifier: Send + Sync {
    fn validate_access_token(&self, token: &str) -> Result<Claims>;
}
