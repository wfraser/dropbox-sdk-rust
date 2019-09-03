// Copyright (c) 2019 Dropbox, Inc.

#![deny(rust_2018_idioms)]

#[macro_use] extern crate failure;
#[macro_use] extern crate log;

#[derive(Debug, Fail)]
pub enum Error<T: std::fmt::Debug + Send + Sync + 'static> {
    /// The API endpoint returned an error.
    /// When the server responds with HTTP 409, the response is JSON-deserialized into the error
    /// type specific to the route being called and returned in this enum variant.
    #[fail(display = "Dropbox API error: {:?}", _0)]
    API(T),

    /// There was a problem with the API request in general.
    #[fail(display = "Dropbox API request error: {}", _0)]
    RequestError(#[cause] RequestError),

    /// Something else went wrong while making a request to the API.
    #[fail(display = "Dropbox API request error: {}", _0)]
    Other(#[cause] failure::Error),
}

// Convenience shorthand for the strongly-typed variant of the error enum.
pub use Error::API as APIErr;

/// The API request failed due to a general error (not route specific).
#[derive(Debug, Fail)]
pub enum RequestError {
    /// The response was malformed in some way.
    #[fail(display = "Dropbox unexpected API error: {}", reason)]
    UnexpectedError { reason: &'static str },

    /// Used when the server responds with HTTP 400.
    #[fail(display = "Dropbox returned 400 Bad Request: {}", message)]
    BadRequest { message: String },

    /// Used when the server responds with HTTP 401.
    #[fail(display = "Dropbox API token is invalid, expired, or revoked: {}", message)]
    InvalidToken { message: String },

    /// Used when the server responds with HTTP 429.
    #[fail(display = "Dropbox denied the request due to rate-limiting: {}", reason)]
    RateLimited { reason: String },

    /// Used when the server responds with HTTP 5xx.
    #[fail(display = "Dropbox API had an internal server error: {}", message)]
    ServerError { message: String },

    /// Used when none of the more specific `RequestError` types is applicable.
    #[fail(display = "Dropbox API returned HTTP {} {} - {}", code, status, body)]
    GeneralHttpError {
        code: u16,
        status: String,
        body: String,
    },
}

impl<T> From<failure::Error> for Error<T>
    where T: std::fmt::Debug + Send + Sync + 'static,
{
    fn from(e: failure::Error) -> Self {
        Self::Other(e)
    }
}

impl<T, D> From<failure::Context<D>> for Error<T>
    where T: std::fmt::Debug + Send + Sync + 'static,
          D: std::fmt::Display + Send + Sync + 'static,
{
    fn from(e: failure::Context<D>) -> Self {
        Self::Other(e.into())
    }
}

impl<T> From<RequestError> for Error<T>
    where T: std::fmt::Debug + Send + Sync + 'static,
{
    fn from(e: RequestError) -> Self {
        Self::RequestError(e)
    }
}

#[cfg(feature = "hyper_client")] mod hyper_client;
#[cfg(feature = "hyper_client")] pub use hyper_client::{
    HyperClient,
    Oauth2AuthorizeUrlBuilder,
    Oauth2Type,
};

pub mod client_trait;
pub(crate) mod client_helpers;

mod generated; // You need to run the Stone generator to create this module.
pub use generated::*;
