//! High-level helpers for common operations involving files.

pub mod list;
pub mod upload;

use std::fmt::{self, Debug, Display};

/// A composite error type representing an error from the API itself, or a client-side error
/// encountered in making the API call.
pub enum Error {
    /// An error returned from the API.
    Api(Box<dyn std::error::Error + Send + Sync>),

    /// A client-side error encountered in making an API call.
    Other(crate::Error),
}

trait RRExt<T, E: std::error::Error> {
    fn combine(self) -> Result<T, Error>;
}

impl<T, E: std::error::Error + Send + Sync + 'static> RRExt<T, E>
    for Result<Result<T, E>, crate::Error>
{
    fn combine(self) -> Result<T, Error> {
        match self {
            Ok(Ok(v)) => Ok(v),
            Ok(Err(e)) => Err(Error::Api(Box::new(e))),
            Err(e) => Err(Error::Other(e)),
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Api(ref e) => Debug::fmt(e, f),
            Self::Other(ref e) => Debug::fmt(e, f),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Api(ref e) => Display::fmt(e, f),
            Self::Other(ref e) => Display::fmt(e, f),
        }
    }
}

impl std::error::Error for Error {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match self {
            Self::Api(e) => Some(e.as_ref()),
            Self::Other(ref e) => Some(e),
        }
    }
}
