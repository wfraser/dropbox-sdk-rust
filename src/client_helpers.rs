// Copyright (c) 2019-2020 Dropbox, Inc.

use std::fmt::Debug;
use crate::Error;
use crate::client_trait::*;
use serde::{Deserialize};
use serde::de::DeserializeOwned;
use serde::ser::Serialize;

// When Dropbox returns an error with HTTP 409 or 429, it uses an implicit JSON object with the
// following structure, which contains the acutal error as a field.
#[derive(Debug, Deserialize)]
struct TopLevelError<T> {
    pub error_summary: String,
    pub user_message: Option<String>,
    pub error: T,
}

#[derive(Debug, Deserialize)]
struct RateLimitedError {
    pub reason: RateLimitedReason,
    pub retry_after: u32,
}

#[derive(Debug, Deserialize)]
#[serde(tag = ".tag", rename_all = "snake_case")]
enum RateLimitedReason {
    TooManyRequests,
    TooManyWriteOperations,
}

/// Does the request and returns a two-level result. The outer result has an error if something
/// went horribly wrong (I/O errors, parse errors, server 500 errors, etc.). The inner result has
/// an error if the server returned one for the request, otherwise it has the deserialized JSON
/// response and the body stream (if any).
#[allow(clippy::too_many_arguments)]
pub async fn request_with_body<ReturnType, ErrorType, Params>(
    client: &impl HttpClient,
    endpoint: Endpoint,
    style: Style,
    function: &'static str,
    params: Params,
    body: Option<BodyStream>,
    range_start: Option<u64>,
    range_end: Option<u64>,
) -> crate::Result<Result<HttpRequestResult<ReturnType>, ErrorType>>
    where ReturnType: DeserializeOwned,
          ErrorType: DeserializeOwned + Debug + Send + Sync + 'static,
          Params: Serialize,
{
    let params_json = serde_json::to_string(&params)?;
    let result = client.request(
        endpoint, style, function, params_json, ParamsType::Json, body, range_start, range_end)
        .await;
    match result {
        Ok(HttpRequestResultRaw { result_json, content_length, body }) => {
            debug!("json: {}", result_json);
            let result_value: ReturnType = serde_json::from_str(&result_json)?;
            Ok(Ok(HttpRequestResult {
                result: result_value,
                content_length,
                body,
            }))
        },
        Err(HttpClientError::HttpError { code, response_body }) => {
            // Try to turn the error into a more specific one.
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
                            Ok(Err(deserialized.error))
                        },
                        Err(de_error) => {
                            error!("Failed to deserialize JSON from API error: {}", de_error);
                            Err(Error::Json(de_error))
                        }
                    }
                },
                429 => {
                    match serde_json::from_str::<TopLevelError<RateLimitedError>>(&response_body) {
                        Ok(deserialized) => {
                            error!("API Rate-Limited: {:?}", deserialized);
                            Err(Error::RateLimited {
                                reason: format!("{:?}", deserialized.error.reason),
                                retry_after_seconds: deserialized.error.retry_after,
                            })
                        }
                        Err(de_error) => {
                            error!("Failed to deserialize JSON from API error: {}", de_error);
                            Err(Error::Json(de_error))
                        }
                    }
                },
                500 ..= 599 => {
                    Err(Error::ServerError(response_body))
                },
                other_code => {
                    Err(Error::UnexpectedHttpError {
                        code: other_code,
                        response_body,
                    })
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
    client: &impl HttpClient,
    endpoint: Endpoint,
    style: Style,
    function: &'static str,
    params: ParamsType,
    body: Option<BodyStream>,
) -> crate::Result<Result<ReturnType, ErrorType>>
    where ReturnType: DeserializeOwned,
          ErrorType: DeserializeOwned + Debug + Send + Sync + 'static,
          ParamsType: Serialize,
{
    request_with_body(client, endpoint, style, function, params, body, None, None)
        .await
        .map(|result| result.map(|HttpRequestResult { result, .. }| result))
}
