// Copyright (c) 2019 Dropbox, Inc.

#![deny(rust_2018_idioms)]
#[macro_use] extern crate failure;
#[macro_use] extern crate log;

#[derive(Debug, Fail)]
enum Error {
    #[fail(display = "Dropbox unexpected API error: {}", reason)]
    UnexpectedError { reason: &'static str },

    #[fail(display = "Dropbox returned 400 Bad Request: {}", message)]
    BadRequest { message: String },

    #[fail(display = "Dropbox API token is invalid, expired, or revoked: {}", message)]
    InvalidToken { message: String },

    #[fail(display = "Dropbox denied the request due to rate-limiting: {}", reason)]
    RateLimited { reason: String },

    #[fail(display = "Dropbox API had an internal server error: {}", message)]
    ServerError { message: String },

    #[fail(display = "Dropbox API returned HTTP {} {} - {}", code, status, json)]
    GeneralHttpError {
        code: u16,
        status: String,
        json: String,
    },
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
