//! HTTP/JSON transport every Gmail coroutine delegates to.
//!
//! Builds the authorized request and parses the JSON response, or the
//! Gmail error envelope on failure.
//!
//! Gmail API reference: <https://developers.google.com/gmail/api/reference/rest>.

use core::marker::PhantomData;

use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use io_http::{
    coroutine::*,
    rfc6750::bearer::HttpAuthBearer,
    rfc9110::{
        request::HttpRequest,
        send::{HttpSendOutput, HttpSendYield},
    },
    rfc9112::send::{Http11Send, Http11SendError},
};
use log::{debug, trace};
use serde::{Deserialize, Deserializer, Serialize, de::DeserializeOwned};
use thiserror::Error;
use url::Url;

use crate::coroutine::*;

/// Base URL of the Gmail REST API v1.
pub const GMAIL_API_BASE: &str = "https://gmail.googleapis.com/gmail/v1/";

/// Unit marker deserialized from empty 2xx bodies (DELETE, batch
/// operations, stop).
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct GmailNoResponse;

impl<'de> Deserialize<'de> for GmailNoResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let _ = serde::de::IgnoredAny::deserialize(deserializer)?;
        Ok(Self)
    }
}

/// Errors that can occur during a Gmail exchange.
#[derive(Debug, Error)]
pub enum GmailSendError {
    /// The underlying HTTP exchange failed.
    #[error("Gmail HTTP request failed: {0}")]
    Send(#[from] Http11SendError),
    /// The request body could not be serialized to JSON.
    #[error("Gmail request serialization failed: {0}")]
    SerializeRequest(#[source] serde_json::Error),
    /// The 2xx response body could not be parsed as JSON.
    #[error("Gmail response parsing failed: {0}")]
    ParseResponse(#[source] serde_json::Error),
    /// The request URL could not be built.
    #[error("Gmail URL parsing failed: {0}")]
    ParseUrl(#[from] url::ParseError),
    /// The request was rejected before being sent.
    #[error("Invalid Gmail request: {0}")]
    InvalidRequest(String),
    /// Gmail returned a non-2xx status with its error envelope.
    #[error("Gmail API returned HTTP {status}: {message}")]
    Api {
        /// The effective status code, from the envelope when present.
        status: u16,
        /// The error message, from the envelope or the raw body.
        message: String,
    },
    /// The server answered with a redirect, which is never followed.
    #[error("Gmail server returned an unexpected redirect")]
    UnexpectedRedirect,
}

impl GmailSendError {
    /// Returns the HTTP status code when the error is an API error.
    pub fn status(&self) -> Option<u16> {
        match self {
            Self::Api { status, .. } => Some(*status),
            _ => None,
        }
    }

    /// Whether the error is transient (429 or 5xx) and worth retrying.
    pub fn is_retryable(&self) -> bool {
        matches!(self.status(), Some(429 | 500 | 502 | 503 | 504))
    }
}

/// Terminal value of a successful Gmail exchange.
#[derive(Clone, Debug)]
pub struct GmailSendOutput<T> {
    /// The parsed 2xx response body.
    pub response: T,
    /// Whether the server allows reusing the TCP/TLS connection.
    pub keep_alive: bool,
}

/// I/O-free coroutine sending one authorized HTTP request and parsing
/// the JSON response into `T`.
pub struct GmailSend<T> {
    state: State,
    _phantom: PhantomData<T>,
}

impl<T: DeserializeOwned> GmailSend<T> {
    /// Builds a GET request against the given URL.
    pub fn get(auth: &HttpAuthBearer, url: Url) -> Self {
        Self::with_method(auth, "GET", url, None, Vec::new())
    }

    /// Builds a DELETE request against the given URL.
    pub fn delete(auth: &HttpAuthBearer, url: Url) -> Self {
        Self::with_method(auth, "DELETE", url, None, Vec::new())
    }

    /// Builds a POST request with the given value as JSON body.
    pub fn post_json<B: Serialize>(
        auth: &HttpAuthBearer,
        url: Url,
        body: &B,
    ) -> Result<Self, GmailSendError> {
        let body = serde_json::to_vec(body).map_err(GmailSendError::SerializeRequest)?;
        Ok(Self::with_method(
            auth,
            "POST",
            url,
            Some("application/json"),
            body,
        ))
    }

    /// Builds a PUT request with the given value as JSON body.
    pub fn put_json<B: Serialize>(
        auth: &HttpAuthBearer,
        url: Url,
        body: &B,
    ) -> Result<Self, GmailSendError> {
        let body = serde_json::to_vec(body).map_err(GmailSendError::SerializeRequest)?;
        Ok(Self::with_method(
            auth,
            "PUT",
            url,
            Some("application/json"),
            body,
        ))
    }

    /// Builds a PATCH request with the given value as JSON body.
    pub fn patch_json<B: Serialize>(
        auth: &HttpAuthBearer,
        url: Url,
        body: &B,
    ) -> Result<Self, GmailSendError> {
        let body = serde_json::to_vec(body).map_err(GmailSendError::SerializeRequest)?;
        Ok(Self::with_method(
            auth,
            "PATCH",
            url,
            Some("application/json"),
            body,
        ))
    }

    /// Builds a request with an arbitrary method, content type and body.
    pub fn with_method(
        auth: &HttpAuthBearer,
        method: &str,
        url: Url,
        content_type: Option<&str>,
        body: Vec<u8>,
    ) -> Self {
        let host = url.host_str().unwrap_or("localhost");

        let mut request = HttpRequest::get(url.clone())
            .header("Host", host)
            .header("Accept", "application/json")
            .header("Authorization", auth.to_authorization())
            .body(body);

        if let Some(content_type) = content_type {
            request = request.header("Content-Type", content_type);
        }

        request.method = method.into();

        debug!("prepare request to send");
        trace!("method: {method}");
        trace!("url: {url}");

        Self {
            state: State::Send(Http11Send::new(request)),
            _phantom: PhantomData,
        }
    }
}

impl<T: DeserializeOwned> GmailCoroutine for GmailSend<T> {
    type Yield = GmailYield;
    type Return = Result<GmailSendOutput<T>, GmailSendError>;

    fn resume(&mut self, arg: Option<&[u8]>) -> GmailCoroutineState<Self::Yield, Self::Return> {
        match &mut self.state {
            State::Send(send) => match send.resume(arg) {
                HttpCoroutineState::Yielded(HttpSendYield::WantsRead) => {
                    GmailCoroutineState::Yielded(GmailYield::WantsRead)
                }
                HttpCoroutineState::Yielded(HttpSendYield::WantsWrite(bytes)) => {
                    GmailCoroutineState::Yielded(GmailYield::WantsWrite(bytes))
                }
                HttpCoroutineState::Yielded(HttpSendYield::WantsRedirect { .. }) => {
                    GmailCoroutineState::Complete(Err(GmailSendError::UnexpectedRedirect))
                }
                HttpCoroutineState::Complete(Err(err)) => {
                    GmailCoroutineState::Complete(Err(err.into()))
                }
                HttpCoroutineState::Complete(Ok(HttpSendOutput {
                    response,
                    keep_alive,
                    ..
                })) => {
                    if response.status.is_success() {
                        // NOTE: an empty 2xx body — a DELETE, or a list
                        // endpoint with an empty collection (e.g.
                        // `settings.filters.list`) — is normalised to `{}`.
                        // `GmailNoResponse` ignores it and struct responses
                        // fall back to their `#[serde(default)]` fields;
                        // `null` would fail every struct response with
                        // "invalid type: null".
                        let body = if response.body.is_empty() {
                            b"{}".as_slice()
                        } else {
                            response.body.as_slice()
                        };

                        match serde_json::from_slice::<T>(body) {
                            Ok(response) => GmailCoroutineState::Complete(Ok(GmailSendOutput {
                                response,
                                keep_alive,
                            })),
                            Err(err) => GmailCoroutineState::Complete(Err(
                                GmailSendError::ParseResponse(err),
                            )),
                        }
                    } else {
                        let (status, message) = parse_api_error(*response.status, &response.body);
                        GmailCoroutineState::Complete(Err(GmailSendError::Api { status, message }))
                    }
                }
            },
        }
    }
}

enum State {
    Send(Http11Send),
}

#[derive(Debug, Deserialize)]
struct ErrorEnvelope {
    error: ErrorBody,
}

#[derive(Debug, Deserialize)]
struct ErrorBody {
    code: Option<u16>,
    message: Option<String>,
}

/// Parses Gmail's JSON error envelope, falling back to the raw body;
/// returns the effective status code and message.
pub fn parse_api_error(http_status: u16, body: &[u8]) -> (u16, String) {
    if let Ok(envelope) = serde_json::from_slice::<ErrorEnvelope>(body) {
        let status = envelope.error.code.unwrap_or(http_status);
        let message = envelope
            .error
            .message
            .filter(|message| !message.trim().is_empty())
            .unwrap_or_else(|| String::from("unknown Gmail API error"));
        return (status, message);
    }

    let message = String::from_utf8_lossy(body).trim().to_string();

    if message.is_empty() {
        (http_status, String::from("unknown Gmail API error"))
    } else {
        (http_status, message)
    }
}
