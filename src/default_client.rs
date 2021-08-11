// Copyright (c) 2020-2021 Dropbox, Inc.

//! The default HTTP client.
//!
//! Use this client if you're not particularly picky about implementation details, as the specific
//! implementation is not exposed, and may be changed in the future.
//!
//! If you have a need for a specific HTTP client implementation, or your program is already using
//! some HTTP client crate, you probably want to have this Dropbox SDK crate use it as well. To do
//! that, you should implement the [`HttpClient`](crate::client_trait::HttpClient) trait for it and
//! use it instead.
//!
//! This code (and its dependencies) are only built if you use the `default_client` Cargo feature.

use crate::Error;
use crate::client_trait::*;
use std::borrow::Cow;

const USER_AGENT: &str = concat!("Dropbox-APIv2-Rust/", env!("CARGO_PKG_VERSION"));

/// A default HTTP client implementation.
///
/// This implementation currently uses the `ureq` crate, but this is subject to change in the
/// future.
#[derive(Debug, Default)]
pub struct DefaultClient {}

impl HttpClient for DefaultClient {
    #[allow(clippy::too_many_arguments)]
    fn request(
        &self,
        endpoint: Endpoint,
        style: Style,
        function: &str,
        params: &str,
        params_type: ParamsType,
        body: Option<&[u8]>,
        range_start: Option<u64>,
        range_end: Option<u64>,
        token: Option<(TokenType, &str)>,
        path_root: Option<&str>,
        team_select: Option<&TeamSelect>,
    ) -> crate::Result<HttpRequestResultRaw> {

        let url = endpoint.url().to_owned() + function;
        debug!("request for {:?}", url);

        let mut req = ureq::post(&url)
            .set("User-Agent", USER_AGENT);

        if let Some((typ, token)) = token {
            req = req.set("Authorization", &format!("{} {}", typ.authorization_type(), token));
        }

        if let Some(path_root) = path_root {
            req = req.set("Dropbox-API-Path-Root", path_root);
        }

        if let Some(team_select) = team_select {
            req = match team_select {
                TeamSelect::User(id) => req.set("Dropbox-API-Select-User", id),
                TeamSelect::Admin(id) => req.set("Dropbox-API-Select-Admin", id),
            };
        }

        req = match (range_start, range_end) {
            (Some(start), Some(end)) => req.set("Range", &format!("bytes={}-{}", start, end)),
            (Some(start), None) => req.set("Range", &format!("bytes={}-", start)),
            (None, Some(end)) => req.set("Range", &format!("bytes=-{}", end)),
            (None, None) => req,
        };

        // If the params are totally empty, don't send any arg header or body.
        let result = if params.is_empty() {
            req.call()
        } else {
            match style {
                Style::Rpc => {
                    // Send params in the body.
                    req = req.set("Content-Type", params_type.content_type());
                    req.send_string(params)
                }
                Style::Upload | Style::Download => {
                    // Send params in a header. Note that non-ASCII and 0x7F in a header need to be
                    // escaped per the HTTP spec.
                    req = req.set(
                        "Dropbox-API-Arg",
                        json_escape_header(params).as_ref());
                    if style == Style::Upload {
                        req = req.set("Content-Type", "application/octet-stream");
                        if let Some(body) = body {
                            req.send_bytes(body)
                        } else {
                            req.send_bytes(&[])
                        }
                    } else {
                        assert!(body.is_none(), "body can only be set for Style::Upload request");
                        req.call()
                    }
                }
            }
        };

        let resp = match result {
            Ok(resp) => resp,
            Err(e @ ureq::Error::Transport(_)) => {
                error!("request failed: {}", e);
                return Err(RequestError { inner: e }.into());
            }
            Err(ureq::Error::Status(code, resp)) => {
                let status = resp.status_text().to_owned();
                let json = resp.into_string()?;
                return Err(Error::UnexpectedHttpError {
                    code,
                    status,
                    json,
                });
            }
        };

        match style {
            Style::Rpc | Style::Upload => {
                // Get the response from the body; return no body stream.
                let result_json = resp.into_string()?;
                Ok(HttpRequestResultRaw {
                    result_json,
                    content_length: None,
                    body: None,
                })
            }
            Style::Download => {
                // Get the response from a header; return the body stream.
                let result_json = resp.header("Dropbox-API-Result")
                    .ok_or(Error::UnexpectedResponse("missing Dropbox-API-Result header"))?
                    .to_owned();

                let content_length = match resp.header("Content-Length") {
                    Some(s) => Some(s.parse()
                        .map_err(|_| Error::UnexpectedResponse("invalid Content-Length header"))?),
                    None => None,
                };

                Ok(HttpRequestResultRaw {
                    result_json,
                    content_length,
                    body: Some(Box::new(resp.into_reader())),
                })
            }
        }
    }
}

/// Errors from the HTTP client encountered in the course of making a request.
#[derive(thiserror::Error, Debug)]
#[allow(clippy::large_enum_variant)] // it's always boxed
pub enum DefaultClientError {
    /// The HTTP client encountered invalid UTF-8 data.
    #[error("invalid UTF-8 string")]
    Utf8(#[from] std::string::FromUtf8Error),

    /// The HTTP client encountered some I/O error.
    #[error("I/O error: {0}")]
    #[allow(clippy::upper_case_acronyms)]
    IO(#[from] std::io::Error),

    /// Some other error from the HTTP client implementation.
    #[error(transparent)]
    Request(#[from] RequestError),
}

macro_rules! wrap_error {
    ($e:ty) => {
        impl From<$e> for crate::Error {
            fn from(e: $e) -> Self {
                Self::HttpClient(Box::new(DefaultClientError::from(e)))
            }
        }
    }
}

wrap_error!(std::io::Error);
wrap_error!(std::string::FromUtf8Error);
wrap_error!(RequestError);

/// Something went wrong making the request, or the server returned a response we didn't expect.
/// Use the `Display` or `Debug` impls to see more details.
/// Note that this type is intentionally vague about the details beyond these string
/// representations, to allow implementation changes in the future.
pub struct RequestError {
    inner: ureq::Error,
}

impl std::fmt::Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <ureq::Error as std::fmt::Display>::fmt(&self.inner, f)
    }
}

impl std::fmt::Debug for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <ureq::Error as std::fmt::Debug>::fmt(&self.inner, f)
    }
}

impl std::error::Error for RequestError {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        Some(&self.inner)
    }
}

/// Replaces any non-ASCII characters (and 0x7f) with JSON-style '\uXXXX' sequence. Otherwise,
/// returns it unmodified without any additional allocation or copying.
fn json_escape_header(s: &str) -> Cow<'_, str> {
    // Unfortunately, the HTTP spec requires escaping ASCII DEL (0x7F), so we can't use the quicker
    // bit pattern check done in str::is_ascii() to skip this for the common case of all ASCII. :(

    let mut out = Cow::Borrowed(s);
    for (i, c) in s.char_indices() {
        if !c.is_ascii() || c == '\x7f' {
            let mstr = match out {
                Cow::Borrowed(_) => {
                    // If we're still borrowed, we must have had ascii up until this point.
                    // Clone the string up until here, and from now on we'll be pushing chars to it.
                    out = Cow::Owned((&s[0..i]).to_owned());
                    out.to_mut()
                }
                Cow::Owned(ref mut m) => m,
            };
            mstr.push_str(&format!("\\u{:04x}", c as u32));
        } else if let Cow::Owned(ref mut o) = out {
            o.push(c);
        }
    }
    out
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_json_escape() {
        assert_eq!(Cow::Borrowed("foobar"), json_escape_header("foobar"));
        assert_eq!(
            Cow::<'_, str>::Owned("tro\\u0161kovi".to_owned()),
            json_escape_header("troškovi"));
        assert_eq!(
            Cow::<'_, str>::Owned(
                r#"{"field": "some_\u00fc\u00f1\u00eec\u00f8d\u00e9_and_\u007f"}"#.to_owned()),
            json_escape_header("{\"field\": \"some_üñîcødé_and_\x7f\"}"));
        assert_eq!(
            Cow::<'_, str>::Owned("almost,\\u007f but not quite".to_owned()),
            json_escape_header("almost,\x7f but not quite"));
    }
}
