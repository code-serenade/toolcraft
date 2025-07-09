use thiserror::Error;
use zip::result::ZipError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("zip error: {0}")]
    ZipError(#[from] ZipError),

    #[error("failed to create graph")]
    GraphCreationError,
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
