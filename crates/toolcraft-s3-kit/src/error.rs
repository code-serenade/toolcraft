use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("request error: {0}")]
    Request(#[from] reqwest::Error),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("url parse error: {0}")]
    Url(#[from] url::ParseError),

    #[error("s3 error {status}: {message}")]
    S3 { status: u16, message: String },

    #[error("{0}")]
    Message(Box<str>),
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
