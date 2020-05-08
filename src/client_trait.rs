// Copyright (c) 2019 Dropbox, Inc.

//! Everything needed to implement your HTTP client.

use std::future::Future;
use std::io::Read;

pub type HttpResult = Result<HttpRequestResultRaw, HttpClientError>;

pub trait HttpClient<F: Future<Output=HttpResult>> {
    #[allow(clippy::too_many_arguments)]
    fn request(
        &self,
        endpoint: Endpoint,
        style: Style,
        function: &str,
        params_json: String,
        body: Option<&[u8]>,
        range_start: Option<u64>,
        range_end: Option<u64>,
    ) -> F;
}

/// An error returned by the HTTP client.
pub enum HttpClientError {
    /// The server responded something other than HTTP 200.
    HttpError {
        code: u16,
        response_body: String,
    },
    /// Some other error occurred in the course of making the HTTP request.
    Other(Box<dyn std::error::Error + Send + Sync + 'static>),
}

impl<E: std::error::Error + Send + Sync + 'static> From<E> for HttpClientError {
    fn from(e: E) -> Self {
        HttpClientError::Other(Box::new(e))
    }
}

pub struct HttpRequestResultRaw {
    pub result_json: String,
    pub content_length: Option<u64>,
    pub body: Option<Box<dyn Read>>,
}

pub struct HttpRequestResult<T> {
    pub result: T,
    pub content_length: Option<u64>,
    pub body: Option<Box<dyn Read>>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Endpoint {
    Api,
    Content,
    Notify,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Style {
    Rpc,
    Upload,
    Download,
}

impl Endpoint {
    pub fn url(self) -> &'static str {
        match self {
            Endpoint::Api => "https://api.dropboxapi.com/2/",
            Endpoint::Content => "https://content.dropboxapi.com/2/",
            Endpoint::Notify => "https://notify.dropboxapi.com/2/",
        }
    }
}
