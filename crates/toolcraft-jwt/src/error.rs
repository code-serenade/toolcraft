use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("JWT error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),

    #[error("auth error: {0}")]
    AuthError(Box<str>),

    #[error("error message: {0}")]
    ErrorMessage(Box<str>),
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
