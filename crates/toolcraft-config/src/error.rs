use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("error message: {0}")]
    ErrorMessage(Box<str>),
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
