// Copyright (c) 2019-2020 Dropbox, Inc.

//! Everything needed to implement your HTTP client.

use async_trait::async_trait;
use bytes::Bytes;
use futures::stream::Stream;

#[async_trait]
pub trait HttpClient {
    #[allow(clippy::too_many_arguments)]
    async fn request(
        &self,
        endpoint: Endpoint,
        style: Style,
        function: &str,
        params_json: String,
        body: Option<Vec<u8>>,  // TODO: allow passing a Stream, reader, or a static buffer instead
        range_start: Option<u64>,
        range_end: Option<u64>,
    ) -> Result<HttpRequestResultRaw, HttpClientError>;
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

impl std::fmt::Display for HttpClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpClientError::HttpError { code, .. } => write!(f, "HTTP {}", code),
            HttpClientError::Other(e) => write!(f, "{}", e),
        }
    }
}

pub struct HttpRequestResultRaw {
    pub result_json: String,
    pub content_length: Option<u64>,
    pub body: Option<Box<dyn Stream<Item=Result<Bytes, HttpClientError>>>>,
}

pub struct HttpRequestResult<T> {
    pub result: T,
    pub content_length: Option<u64>,
    pub body: Option<Box<dyn Stream<Item=Result<Bytes, HttpClientError>>>>,
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
