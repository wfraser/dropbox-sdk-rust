// Copyright (c) 2019-2020 Dropbox, Inc.

use std::fmt::Debug;
use std::marker::PhantomData;
use crate::Error;
use crate::client_trait::*;
use serde::de::{self, Deserialize, DeserializeOwned, Deserializer, MapAccess, Visitor};
use serde::ser::Serialize;

// When Dropbox returns an error with HTTP 409, it uses an implicit JSON object with the following
// structure, which contains the acutal error as a field.
#[derive(Debug)]
struct TopLevelError<T> {
    pub error_summary: String,
    pub user_message: Option<String>,
    pub error: T,
}

impl<'de, T: DeserializeOwned> Deserialize<'de> for TopLevelError<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct StructVisitor<T> {
            phantom: PhantomData<T>,
        }
        impl<'de, T: DeserializeOwned> Visitor<'de> for StructVisitor<T> {
            type Value = TopLevelError<T>;
            fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str("a top-level error struct")
            }
            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
                let mut error_summary = None;
                let mut user_message = None;
                let mut error = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "error_summary" => {
                            if error_summary.is_some() {
                                return Err(de::Error::duplicate_field("error_summary"));
                            }
                            error_summary = Some(map.next_value()?);
                        }
                        "user_message" => {
                            if user_message.is_some() {
                                return Err(de::Error::duplicate_field("user_message"));
                            }
                            user_message = Some(map.next_value()?);
                        }
                        "error" => {
                            if error.is_some() {
                                return Err(de::Error::duplicate_field("error"));
                            }
                            error = Some(map.next_value()?);
                        }
                        _ => return Err(de::Error::unknown_field(key, FIELDS))
                    }
                }
                Ok(TopLevelError {
                    error_summary: error_summary.ok_or_else(|| de::Error::missing_field("error_summary"))?,
                    user_message,
                    error: error.ok_or_else(|| de::Error::missing_field("error"))?,
                })
            }
        }
        const FIELDS: &[&str] = &["error_summary", "user_message", "error"];
        deserializer.deserialize_struct("<top level error>", FIELDS, StructVisitor { phantom: PhantomData })
    }
}

/// Does the request and returns a two-level result. The outer result has an error if something
/// went horribly wrong (I/O errors, parse errors, server 500 errors, etc.). The inner result has
/// an error if the server returned one for the request, otherwise it has the deserialized JSON
/// response and the body stream (if any).
#[allow(clippy::too_many_arguments)]
pub async fn request_with_body<ReturnType, ErrorType, ParamsType>(
    client: &dyn HttpClient,
    endpoint: Endpoint,
    style: Style,
    function: &str,
    params: &ParamsType,
    body: Option<Vec<u8>>,
    range_start: Option<u64>,
    range_end: Option<u64>,
) -> crate::Result<HttpRequestResult<ReturnType>, ErrorType>
    where ReturnType: DeserializeOwned,
          ErrorType: DeserializeOwned + Debug + Send + Sync + 'static,
          ParamsType: Serialize,
{
    let params_json = serde_json::to_string(params)?;
    let result = client.request(
        endpoint, style, function, params_json, body, range_start, range_end)
        .await;
    match result {
        Ok(HttpRequestResultRaw { result_json, content_length, body }) => {
            debug!("json: {}", result_json);
            let result_value: ReturnType = serde_json::from_str(&result_json)?;
            Ok(HttpRequestResult {
                result: result_value,
                content_length,
                body,
            })
        },
        Err(HttpClientError::HttpError { code, response_body }) => {
            error!("HTTP {}: {}", code, response_body);
            match code {
                400 => {
                    Err(Error::BadRequest(response_body))
                },
                401 => {
                    Err(Error::InvalidToken(response_body))
                },
                409 => {
                    // Response should be JSON-deseraializable into the strongly-typed
                    // error specified by type parameter ErrorType.
                    match serde_json::from_str::<TopLevelError<ErrorType>>(&response_body) {
                        Ok(deserialized) => {
                            error!("API error: {:?}", deserialized);
                            Err(Error::API(deserialized.error))
                        },
                        Err(de_error) => {
                            error!("Failed to deserialize JSON from API error: {}", de_error);
                            Err(Error::Json(de_error))
                        }
                    }
                },
                429 => {
                    Err(Error::RateLimited(response_body))
                },
                500 ..= 599 => {
                    Err(Error::ServerError(response_body))
                },
                _ => {
                    Err(Error::UnexpectedHttpError { code, response_body })
                }
            }
        }
        Err(HttpClientError::Other(e)) => {
            error!("HTTP request error: {}", e);
            Err(Error::HttpClient(e))
        }
    }
}

pub async fn request<ReturnType, ErrorType, ParamsType>(
    client: &dyn HttpClient,
    endpoint: Endpoint,
    style: Style,
    function: &str,
    params: &ParamsType,
    body: Option<Vec<u8>>,
) -> crate::Result<ReturnType, ErrorType>
    where ReturnType: DeserializeOwned,
          ErrorType: DeserializeOwned + Debug + Send + Sync + 'static,
          ParamsType: Serialize,
{
    request_with_body(client, endpoint, style, function, params, body, None, None)
        .await
        .map(|HttpRequestResult { result, .. }| result)
}
