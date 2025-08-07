use thiserror::Error;
use zip::result::ZipError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("zip error: {0}")]
    ZipError(#[from] ZipError),

    #[error("Invalid UTF8 sequence: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("error message: {0}")]
    ErrorMessage(Box<str>),
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
