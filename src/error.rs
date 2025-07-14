use thiserror::Error;
use zip::result::ZipError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("zip error: {0}")]
    ZipError(#[from] ZipError),

    #[error("request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("url error: {0}")]
    UrlError(#[from] url::ParseError),

    #[error("failed to create graph")]
    GraphCreationError,

    #[error("error message: {0}")]
    ErrorMessage(String),
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
