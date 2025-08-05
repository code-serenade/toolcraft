use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid UTF8 sequence: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("url error: {0}")]
    UrlError(#[from] url::ParseError),

    #[error("error message: {0}")]
    ErrorMessage(Box<str>),
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
