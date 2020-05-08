// Copyright (c) 2020 Dropbox, Inc.

//! The default HTTP client.
//!
//! Use this client if you're not particularly picky about implementation details, as the specific
//! implementation is not exposed, and may be changed in the future.
//!
//! If you have a need for a specific HTTP client implementation, or your program is already using
//! some HTTP client crate, you probably want to have this Dropbox SDK crate use it as well. To do
//! that, you should implement the traits in `crate::client_trait` for it and use it instead.
//!
//! This code (and its dependencies) are only built if you use the `default_client` Cargo feature.

use async_trait::async_trait;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use crate::client_trait::{BodyStream, Endpoint, Style, HttpClient, HttpClientError,
    HttpRequestResultRaw, NoauthClient, ParamsType, TeamAuthClient, TeamSelect, UserAuthClient};
use futures::io::{AsyncRead, AsyncBufRead};
use futures::stream::{Stream, StreamExt};
use hyper::{Body, Request, Uri};
use hyper::header::{self, HeaderValue};
use pin_project::pin_project;
use std::convert::TryFrom;
use std::str;
use std::pin::Pin;
use std::task::{Context, Poll};

const USER_AGENT: &str = concat!("Dropbox-APIv2-Rust/", env!("CARGO_PKG_VERSION"));

/// Default HTTP client for unauthenticated API calls.
#[derive(Default)]
pub struct NoauthDefaultClient {
    inner: HyperClient,
}

#[async_trait]
impl HttpClient for NoauthDefaultClient {
    #[allow(clippy::too_many_arguments)]
    async fn request(
        &self,
        endpoint: Endpoint,
        style: Style,
        function: &'static str,
        params: String,
        params_type: ParamsType,
        body: Option<BodyStream>,
        range_start: Option<u64>,
        range_end: Option<u64>,
    ) -> Result<HttpRequestResultRaw, HttpClientError> {
        self.inner.request(endpoint, style, function, params, params_type, body, range_start,
            range_end, None, None)
            .await
    }
}

impl NoauthClient for NoauthDefaultClient {}

/// Default HTTP client using User authorization.
pub struct UserAuthDefaultClient {
    inner: HyperClient,
    token: String,
}

impl UserAuthDefaultClient {
    /// Create a new client using the given OAuth2 token.
    pub fn new(token: String) -> Self {
        Self {
            inner: HyperClient::default(),
            token,
        }
    }
}

#[async_trait]
impl HttpClient for UserAuthDefaultClient {
    #[allow(clippy::too_many_arguments)]
    async fn request(
        &self,
        endpoint: Endpoint,
        style: Style,
        function: &'static str,
        params: String,
        params_type: ParamsType,
        body: Option<BodyStream>,
        range_start: Option<u64>,
        range_end: Option<u64>,
    ) -> Result<HttpRequestResultRaw, HttpClientError> {
        self.inner.request(endpoint, style, function, params, params_type, body, range_start,
            range_end, Some(self.token.clone()), None)
            .await
    }
}

impl UserAuthClient for UserAuthDefaultClient {}

/// Default HTTP client using Team authorization.
pub struct TeamAuthDefaultClient {
    inner: HyperClient,
    token: String,
    team_select: Option<TeamSelect>,
}

impl TeamAuthDefaultClient {
    /// Create a new client using the given OAuth2 token, with no user/admin context selected.
    pub fn new(token: String) -> Self {
        Self {
            inner: HyperClient::default(),
            token,
            team_select: None,
        }
    }

    /// Select a user or team context to operate in.
    pub fn select(&mut self, team_select: Option<TeamSelect>) {
        self.team_select = team_select;
    }
}

#[async_trait]
impl HttpClient for TeamAuthDefaultClient {
    #[allow(clippy::too_many_arguments)]
    async fn request(
        &self,
        endpoint: Endpoint,
        style: Style,
        function: &'static str,
        params: String,
        params_type: ParamsType,
        body: Option<BodyStream>,
        range_start: Option<u64>,
        range_end: Option<u64>,
    ) -> Result<HttpRequestResultRaw, HttpClientError> {
        self.inner.request(endpoint, style, function, params, params_type, body, range_start,
            range_end, Some(self.token.clone()), self.team_select.clone())
            .await
    }
}

impl TeamAuthClient for TeamAuthDefaultClient {}

/// Errors from the HTTP client encountered in the course of making a request.
#[derive(thiserror::Error, Debug)]
pub enum DefaultHttpClientError {
    /// The HTTP client encountered invalid UTF-8 data.
    #[error("Invalid UTF-8 string")]
    Utf8(#[from] std::string::FromUtf8Error),

    /// The HTTP client encountered some I/O error.
    #[error("I/O error: {0}")]
    IO(#[from] std::io::Error),

    /// Some other error from the HTTP client implementation.
    #[error(transparent)]
    Hyper(#[from] hyper::Error),

    /// THe response is missing the Dropbox-API-Result header.
    #[error("missing Dropbox-API-Result header")]
    MissingHeader,
}

// Implement From for some errors so that they get wrapped in a DefaultHttpClientError and then
// propogated via Error::HttpClient. Note that this only works for types that don't already have a
// variant in the crate Error type, because doing so would produce a conflicting impl.
macro_rules! hyper_error {
    ($e:ty) => {
        impl From<$e> for crate::Error {
            fn from(e: $e) -> Self {
                Self::HttpClient(Box::new(DefaultHttpClientError::from(e)))
            }
        }
    }
}

hyper_error!(std::io::Error);
hyper_error!(std::string::FromUtf8Error);
hyper_error!(hyper::Error);

// Common HTTP client:

type Client = hyper::client::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>;

fn http_client() -> Client {
    let https = hyper_tls::HttpsConnector::new();
    hyper::client::Client::builder()
        .build::<_, hyper::Body>(https)
}

struct HyperClient {
    client: Client,
}

impl Default for HyperClient {
    fn default() -> Self {
        Self {
            client: http_client(),
        }
    }
}

impl HyperClient {
    #[allow(clippy::too_many_arguments)]
    pub async fn request(
        &self,
        endpoint: Endpoint,
        style: Style,
        function: &'static str,
        params: String,
        params_type: ParamsType,
        body: Option<BodyStream>,
        range_start: Option<u64>,
        range_end: Option<u64>,
        token: Option<String>,
        team_select: Option<TeamSelect>,
    ) -> Result<HttpRequestResultRaw, HttpClientError> {

        let uri = Uri::try_from(endpoint.url().to_owned() + function)
            .expect("invalid request URL");
        debug!("request for {:?}", uri);

        let mut builder = Request::builder()
            .method("POST")
            .uri(uri)
            .header(header::USER_AGENT, USER_AGENT)
            .header(header::CONNECTION, "keep-alive");

        if let Some(token) = token {
            builder = builder.header(header::AUTHORIZATION, format!("Bearer {}", token));
        }
        if let Some(team_select) = team_select {
            let name = team_select.header_name();
            let value = match team_select {
                TeamSelect::User(id) => id,
                TeamSelect::Admin(id) => id,
            };
            builder = builder.header(name, HeaderValue::try_from(value).unwrap());
        }

        let request = {
            let range = if let Some(start) = range_start {
                if let Some(end) = range_end {
                    Some(format!("bytes={}-{}", start, end))
                } else {
                    Some(format!("bytes={}-", start))
                }
            } else if let Some(end) = range_end {
                Some(format!("bytes=-{}", end))
            } else {
                None
            };

            if let Some(range) = range {
                builder = builder.header(header::RANGE, HeaderValue::try_from(range).unwrap());
            }

            let request_body = match style {
                Style::Rpc => {
                    // Send params in the body.
                    assert!(body.is_none());
                    if !params.is_empty() {
                        builder = builder.header(header::CONTENT_TYPE, params_type.content_type());
                        Body::from(params)
                    } else {
                        Body::empty()
                    }
                }
                Style::Upload | Style::Download => {
                    // Send params in a header.
                    if !params.is_empty() {
                        builder = builder.header("Dropbox-API-Arg", params);
                    }
                    if style == Style::Upload {
                        builder = builder.header(header::CONTENT_TYPE, "application/octet-stream");
                        match body {
                            Some(body) => Body::wrap_stream(HyperBody(body)),
                            None => Body::empty(),
                        }
                    } else {
                        assert!(body.is_none());
                        Body::empty()
                    }
                }
            };

            builder.body(request_body).map_err(|e| {
                error!("failed to construct HTTP request: {}", e);
                HttpClientError::Other(Box::new(e))
            })?
        };

        let response = match self.client.request(request).await {
            Ok(resp) => resp,
            Err(e) => {
                error!("HTTP request failed: {}", e);
                return Err(HttpClientError::Other(Box::new(e)));
            }
        };

        if !response.status().is_success() {
            let code = response.status().as_u16();
            let response_body = response_body_to_string(response)
                .await
                .map_err(HttpClientError::Other)?;
            return Err(HttpClientError::HttpError { code, response_body });
        }

        match style {
            Style::Rpc | Style::Upload => {
                // Get the response from the body; return no body stream.
                let result_json = response_body_to_string(response)
                    .await
                    .map_err(HttpClientError::Other)?;
                Ok(HttpRequestResultRaw {
                    result_json,
                    content_length: None,
                    body: None,
                })
            },
            Style::Download => {
                // Get the response from a header; return the body stream.
                let s = match response.headers().get("Dropbox-API-Result") {
                    Some(value) => {
                        String::from_utf8(value.as_bytes().to_vec())?
                    },
                    None => {
                        return Err(HttpClientError::Other(Box::new(
                                DefaultHttpClientError::MissingHeader)));
                    }
                };

                let len = response.headers().get(header::CONTENT_LENGTH)
                    .and_then(|val| val.to_str().ok())
                    .and_then(|val| val.parse::<u64>().ok());

                let response_body = Box::pin(
                    BytesStreamToAsyncRead::new(response.into_body().fuse())
                    );

                Ok(HttpRequestResultRaw {
                    result_json: s,
                    content_length: len,
                    body: Some(response_body),
                })
            }
        }
    }
}

/// Adapts a futures::io::AsyncRead to work with hyper::Body by implementing a Stream of Bytes.
#[pin_project]
#[must_use]
struct HyperBody<R>(#[pin] R);

impl<R: AsyncRead> Stream for HyperBody<R> {
    type Item = Result<Bytes, futures::io::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        use std::mem::MaybeUninit;
        let mut inner = self.as_mut().project().0;
        let mut buf = BytesMut::with_capacity(16384);

        // This is basically tokio::io::AsyncRead methods prepare_uninitialized_buffer() and
        // poll_read_buf() put together.
        unsafe {
            let n = {
                let b = buf.bytes_mut();
                for x in b.iter_mut() {
                    *x = MaybeUninit::new(0);
                }

                let b = &mut *(b as *mut [MaybeUninit<u8>] as *mut [u8]);

                let n = match Pin::new(&mut inner).poll_read(cx, b) {
                    Poll::Ready(Ok(0)) => { return Poll::Ready(None); }
                    Poll::Ready(Ok(n)) => n,
                    Poll::Ready(Err(e)) => { return Poll::Ready(Some(Err(e))); }
                    Poll::Pending => { return Poll::Pending; }
                };

                assert!(n <= b.len(), "AsyncRead returned more bytes than there is space for");
                n
            };

            buf.advance_mut(n);
        }

        Poll::Ready(Some(Ok(buf.freeze())))
    }
}

/// Adapts a Hyper body (a stream of Bytes buffers) into an AsyncRead and AsyncBufRead.
/// The AsyncBufRead implementation does not require any copying and should be used if possible.
#[must_use]
struct BytesStreamToAsyncRead<S> {
    stream: S,
    buf: Bytes,
}

impl<S> Unpin for BytesStreamToAsyncRead<S> {}

impl<S> BytesStreamToAsyncRead<S> {
    pub fn new(stream: S) -> Self {
        Self {
            stream,
            buf: Bytes::new(),
        }
    }
}

impl<S> AsyncRead for BytesStreamToAsyncRead<S>
    where S: Stream<Item = Result<Bytes, hyper::Error>> + Unpin
{
    fn poll_read(mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf_out: &mut [u8])
        -> Poll<Result<usize, futures::io::Error>>
    {
        let result = match Pin::new(&mut self).poll_fill_buf(cx) {
            Poll::Ready(Ok(buf_in)) => {
                // Copy the returned buffer into the caller's buffer.
                let len = std::cmp::min(buf_in.len(), buf_out.len());
                (&mut buf_out[0..len]).copy_from_slice(&buf_in[0..len]);
                Poll::Ready(Ok(len))
            }
            Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
            Poll::Pending => Poll::Pending,
        };

        if let Poll::Ready(Ok(len)) = result {
            self.consume(len);
        }

        result
    }
}

impl<S> AsyncBufRead for BytesStreamToAsyncRead<S>
    where S: Stream<Item = Result<Bytes, hyper::Error>> + Unpin
{
    fn poll_fill_buf<'a>(mut self: Pin<&'a mut Self>, cx: &mut Context<'_>)
        -> Poll<Result<&'a [u8], futures::io::Error>>
    {
        if self.buf.is_empty() {
            // Attempt to fill the buffer.
            match Pin::new(&mut self.stream).poll_next(cx) {
                Poll::Ready(Some(Ok(new_buf))) => {
                    // Take over the returned buffer.
                    self.buf = new_buf;
                }
                Poll::Ready(Some(Err(e))) => {
                    // TODO: map the error better
                    return Poll::Ready(Err(futures::io::Error::new(
                                futures::io::ErrorKind::Other,
                                e)));
                }
                Poll::Ready(None) => return Poll::Ready(Ok(&[])),
                Poll::Pending => return Poll::Pending,
            }
        }

        // Return the buffer as a byte slice.
        Poll::Ready(Ok(&self.into_ref().get_ref().buf))
    }

    fn consume(mut self: Pin<&mut Self>, amt: usize) {
        self.buf.advance(amt);
    }
}

// Read a full Hyper body into a String.
async fn response_body_to_string(response: hyper::Response<hyper::Body>)
    -> Result<String, Box<dyn std::error::Error + Send + Sync + 'static>>
{
    let mut bytes = vec![];
    let mut stream = response.into_body();
    while let Some(result) = stream.next().await {
        match result {
            Ok(buf) => {
                bytes.extend(buf);
            }
            Err(e) => {
                return Err(Box::new(e));
            }
        }
    }
    match String::from_utf8(bytes) {
        Ok(body) => Ok(body),
        Err(e) => Err(Box::new(e)),
    }
}
