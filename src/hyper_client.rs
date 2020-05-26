// Copyright (c) 2019-2020 Dropbox, Inc.

use async_trait::async_trait;
use bytes::{Buf, Bytes};
use crate::Error;
use crate::client_trait::{Endpoint, Style, HttpClient, HttpClientError, HttpRequestResultRaw};
use crate::client_trait::BodyStream;
use futures::io::{AsyncRead, AsyncBufRead};
use futures::stream::{Stream, StreamExt};
use hyper::{Body, Request, Uri};
use hyper::header::{self, HeaderValue};
use pin_project::pin_project;
use std::convert::TryFrom;
use std::str;
use std::pin::Pin;
use std::task::{Context, Poll};
use url::form_urlencoded::Serializer as UrlEncoder;

const USER_AGENT: &str = concat!("Dropbox-APIv2-Rust/", env!("CARGO_PKG_VERSION"));

type Client = hyper::client::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>;

pub struct HyperClient {
    client: Client,
    token: String,
}

#[derive(thiserror::Error, Debug)]
pub enum HyperClientError {
    #[error("Invalid UTF-8 string")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("I/O error: {0}")]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    Hyper(#[from] hyper::Error),

    #[error("missing Dropbox-API-Result header")]
    MissingHeader,
}

// Implement From for some errors so that they get wrapped in a HyperClientError and then
// propogated via Error::HttpClient. Note that this only works for types that don't already have a
// variant in the crate Error type, because doing so would produce a conflicting impl.
macro_rules! hyper_error {
    ($e:ty) => {
        impl From<$e> for crate::Error {
            fn from(e: $e) -> Self {
                Self::HttpClient(Box::new(HyperClientError::from(e)))
            }
        }
    }
}

hyper_error!(std::io::Error);
hyper_error!(std::string::FromUtf8Error);
hyper_error!(hyper::Error);

impl HyperClient {
    pub fn new(token: String) -> HyperClient {
        HyperClient {
            client: Self::http_client(),
            token,
        }
    }

    /// Given an authorization code, request an OAuth2 token from Dropbox API.
    /// Requires the App ID and secret, as well as the redirect URI used in the prior authorize
    /// request, if there was one.
    pub async fn oauth2_token_from_authorization_code(
        client_id: &str,
        client_secret: &str,
        authorization_code: &str,
        redirect_uri: Option<&str>,
    ) -> crate::Result<String> {

        let uri = Uri::from_static("https://api.dropboxapi.com/oauth2/token");

        // This endpoint wants parameters using URL-encoding instead of JSON.
        let mut params = UrlEncoder::new(String::new());
        params.append_pair("code", authorization_code);
        params.append_pair("grant_type", "authorization_code");
        params.append_pair("client_id", client_id);
        params.append_pair("client_secret", client_secret);
        if let Some(value) = redirect_uri {
            params.append_pair("redirect_uri", value);
        }

        let request = Request::builder()
            .method("POST")
            .uri(uri)
            .header(header::USER_AGENT, USER_AGENT)
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(Body::from(params.finish()))
            .map_err(|e| {
                error!("failed to construct HTTP request: {}", e);
                Error::HttpClient(Box::new(e))
            })?;

        let client = Self::http_client();
        match client.request(request).await {
            Ok(response) => {
                if !response.status().is_success() {
                    let code = response.status().as_u16();
                    let response_body = response_body_to_string(response)
                        .await
                        .map_err(Error::HttpClient)?;
                    debug!("error body: {}", response_body);
                    Err(Error::UnexpectedHttpError { code, response_body })
                } else {
                    let body = response_body_to_string(response)
                        .await
                        .map_err(Error::HttpClient)?;
                    let json = serde_json::from_str(&body)?;
                    debug!("response: {:?}", json);
                    match json {
                        serde_json::Value::Object(mut map) => {
                            match map.remove("access_token") {
                                Some(serde_json::Value::String(token)) => Ok(token),
                                _ => Err(Error::UnexpectedResponse("no access token in response!")),
                            }
                        }
                        _ => Err(Error::UnexpectedResponse("response is not a JSON object")),
                    }
                }
            }
            Err(e) => {
                error!("error getting OAuth2 token: {}", e);
                Err(e.into())
            }
        }
    }

    fn http_client() -> Client {
        let https = hyper_tls::HttpsConnector::new();
        hyper::client::Client::builder()
            .build::<_, hyper::Body>(https)
    }
}

#[async_trait]
impl HttpClient for HyperClient {
    #[allow(clippy::too_many_arguments)]
    async fn request(
        &self,
        endpoint: Endpoint,
        style: Style,
        function: &'static str,
        params_json: String,
        body: Option<BodyStream<'static>>,
        range_start: Option<u64>,
        range_end: Option<u64>,
    ) -> Result<HttpRequestResultRaw<'static>, HttpClientError> {

        let uri = Uri::try_from(endpoint.url().to_owned() + function)
            .expect("invalid request URL");
        debug!("request for {:?}", uri);

        let mut builder = Request::builder()
            .method("POST")
            .uri(uri)
            .header(header::USER_AGENT, USER_AGENT)
            .header(header::AUTHORIZATION, format!("Bearer {}", self.token))
            .header(header::CONNECTION, "keep-alive");

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
                    if !params_json.is_empty() {
                        builder = builder.header(header::CONTENT_TYPE, "application/json");
                        Body::from(params_json)
                    } else {
                        Body::empty()
                    }
                }
                Style::Upload | Style::Download => {
                    // Send params in a header.
                    if !params_json.is_empty() {
                        builder = builder.header("Dropbox-API-Arg", params_json);
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
                                HyperClientError::MissingHeader)));
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

#[pin_project]
#[must_use]
struct HyperBody<R>(#[pin] R);

impl<R: AsyncBufRead> Stream for HyperBody<R> {
    type Item = Result<Bytes, futures::io::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut inner = self.as_mut().project().0;
        let result = match Pin::new(&mut inner).poll_fill_buf(cx) {
            Poll::Ready(Ok(buf_in)) => {
                if buf_in.is_empty() {
                    Poll::Ready(None)
                } else {
                    Poll::Ready(Some(Ok(Bytes::copy_from_slice(buf_in))))
                }
            }
            Poll::Ready(Err(e)) => {
                Poll::Ready(Some(Err(e)))
            }
            Poll::Pending => Poll::Pending,
        };

        if let Poll::Ready(Some(Ok(ref buf))) = result {
            inner.consume(buf.len());
        }

        result
    }
}

/// Adapts a Hyper body (a stream of Bytes buffers) into an AsyncRead and AsyncBufRead.
/// The AsyncBufRead implementation does not require any copying and should be used if possible.
#[pin_project]
#[must_use]
struct BytesStreamToAsyncRead<S> {
    #[pin]
    stream: S,

    buf: Bytes,
}

impl<S> BytesStreamToAsyncRead<S> {
    pub fn new(stream: S) -> Self {
        Self {
            stream,
            buf: Bytes::new(),
        }
    }
}

impl<S> AsyncRead for BytesStreamToAsyncRead<S>
    where S: Stream<Item = Result<Bytes, hyper::Error>>
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
    where S: Stream<Item = Result<Bytes, hyper::Error>>
{
    fn poll_fill_buf<'a>(self: Pin<&'a mut Self>, cx: &mut Context<'_>)
        -> Poll<Result<&'a [u8], futures::io::Error>>
    {
        // Can't use pin-project here because we need to return a reference to Self at the end, and
        // using pin-project results in an error that we returned a reference to a local.
        // Instead: do basically what pin-project does, but separate out the members.
        let this: &'a mut Self = unsafe { self.get_unchecked_mut() };
        let stream = unsafe { Pin::new_unchecked(&mut this.stream) };

        if this.buf.is_empty() {
            // Attempt to fill the buffer.
            match stream.poll_next(cx) {
                Poll::Ready(Some(Ok(new_buf))) => {
                    // Take over the returned buffer.
                    this.buf = new_buf;
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
        Poll::Ready(Ok(&this.buf))
    }

    fn consume(self: Pin<&mut Self>, amt: usize) {
        self.project().buf.advance(amt);
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

/// Builds a URL that can be given to the user to visit to have Dropbox authorize your app.
#[derive(Debug)]
pub struct Oauth2AuthorizeUrlBuilder<'a> {
    client_id: &'a str,
    response_type: &'a str,
    force_reapprove: bool,
    force_reauthentication: bool,
    disable_signup: bool,
    redirect_uri: Option<&'a str>,
    state: Option<&'a str>,
    require_role: Option<&'a str>,
    locale: Option<&'a str>,
}

/// Which type of OAuth2 flow to use.
#[derive(Debug, Copy, Clone)]
pub enum Oauth2Type {
    /// Authorization yields a temporary authorization code which must be turned into an OAuth2
    /// token by making another call. This can be used without a redirect URI, where the user
    /// inputs the code directly into the program.
    AuthorizationCode,

    /// Authorization directly returns an OAuth2 token. This can only be used with a redirect URI
    /// where the Dropbox server redirects the user's web browser to the program.
    ImplicitGrant,
}

impl Oauth2Type {
    pub fn as_str(self) -> &'static str {
        match self {
            Oauth2Type::AuthorizationCode => "code",
            Oauth2Type::ImplicitGrant => "token",
        }
    }
}

impl<'a> Oauth2AuthorizeUrlBuilder<'a> {
    pub fn new(client_id: &'a str, oauth2_type: Oauth2Type) -> Self {
        Self {
            client_id,
            response_type: oauth2_type.as_str(),
            force_reapprove: false,
            force_reauthentication: false,
            disable_signup: false,
            redirect_uri: None,
            state: None,
            require_role: None,
            locale: None,
        }
    }

    pub fn force_reapprove(mut self, value: bool) -> Self {
        self.force_reapprove = value;
        self
    }

    pub fn force_reauthentication(mut self, value: bool) -> Self {
        self.force_reauthentication = value;
        self
    }

    pub fn disable_signup(mut self, value: bool) -> Self {
        self.disable_signup = value;
        self
    }

    pub fn redirect_uri(mut self, value: &'a str) -> Self {
        self.redirect_uri = Some(value);
        self
    }

    pub fn state(mut self, value: &'a str) -> Self {
        self.state = Some(value);
        self
    }

    pub fn require_role(mut self, value: &'a str) -> Self {
        self.require_role = Some(value);
        self
    }

    pub fn locale(mut self, value: &'a str) -> Self {
        self.locale = Some(value);
        self
    }

    pub fn build(self) -> Uri {
        let mut params = vec![
            ("response_type", self.response_type),
            ("client_id", self.client_id)
        ];

        {
            if self.force_reapprove {
                params.push(("force_reapprove", "true"));
            }
            if self.force_reauthentication {
                params.push(("force_reauthentication", "true"));
            }
            if self.disable_signup {
                params.push(("disable_signup", "true"));
            }
            if let Some(value) = self.redirect_uri {
                params.push(("redirect_uri", value));
            }
            if let Some(value) = self.state {
                params.push(("state", value));
            }
            if let Some(value) = self.require_role {
                params.push(("require_role", value));
            }
            if let Some(value) = self.locale {
                params.push(("locale", value));
            }
        }
        let url = url::Url::parse_with_params("https://www.dropbox.com/oauth2/authorize", params)
            .unwrap()
            .to_string();
        Uri::try_from(url).unwrap()
    }
}
