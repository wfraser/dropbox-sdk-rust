// Copyright (c) 2019-2020 Dropbox, Inc.

#![deny(rust_2018_idioms)]

#[macro_use] extern crate log;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error<T: std::fmt::Debug + Send + Sync + 'static> {

    /// Error returned from the API, of the type specified by the spec.
    #[error("Dropbox API error: {0:?}")]
    API(T),

    /// An error occurred in the course of making the HTTP request.
    #[error("error from HTTP client: {0}")]
    HttpClient(Box<dyn std::error::Error + Send + Sync + 'static>),

    /// Error serializing to or deserializing from JSON.
    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    /// Malformed response from the API.
    #[error("Dropbox API returned something unexpected: {0}")]
    UnexpectedResponse(&'static str),

    /// Server said our request was malformed in some way.
    #[error("Dropbox API indicated that the request was malformed: {0}")]
    BadRequest(String),

    /// The access token (OAUTH2) is invalid in some way.
    #[error("Dropbox API indicated that the access token is bad: {0}")]
    InvalidToken(String),

    /// Server wants us to take a coffee break. You should handle this with some exponential
    /// backoff.
    #[error("Dropbox API declined the request due to rate-limiting: {0}")]
    RateLimited(String),

    /// Dropbox had an internal problem with the request.
    #[error("Dropbox API had an internal server error: {0}")]
    ServerError(String),

    /// Some unexpected HTTP response code was received.
    #[error("Dropbox API returned HTTP {code}: {response_body}")]
    UnexpectedHttpError {
        code: u16,
        response_body: String,
    },
}

pub type Result<T, E> = std::result::Result<T, Error<E>>;

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
