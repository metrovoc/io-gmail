use alloc::{format, string::String, vec::Vec};
use core::marker::PhantomData;

use io_http::{
    rfc9110::request::HttpRequest,
    rfc9112::send::{Http11Send, Http11SendError, Http11SendResult},
};
use io_socket::io::{SocketInput, SocketOutput};
use log::info;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Deserializer, Serialize, de::DeserializeOwned};
use thiserror::Error;
use url::Url;

use crate::error::parse_api_error;

pub const GMAIL_API_BASE: &str = "https://gmail.googleapis.com/gmail/v1/";
pub const GMAIL_UPLOAD_BASE: &str = "https://gmail.googleapis.com/upload/gmail/v1/";

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct NoResponse;

impl<'de> Deserialize<'de> for NoResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let _ = serde::de::IgnoredAny::deserialize(deserializer)?;
        Ok(Self)
    }
}

#[derive(Debug, Error)]
pub enum GmailSendError {
    #[error("Send HTTP request error: {0}")]
    SendHttp(#[from] Http11SendError),
    #[error("Serialize Gmail request error: {0}")]
    SerializeRequest(#[source] serde_json::Error),
    #[error("Parse Gmail response error: {0}")]
    ParseResponse(#[source] serde_json::Error),
    #[error("Parse Gmail URL error: {0}")]
    ParseUrl(#[from] url::ParseError),
    #[error("Invalid Gmail request: {0}")]
    InvalidRequest(String),
    #[error("Gmail API returned HTTP {status}: {message}")]
    ApiError { status: u16, message: String },
    #[error("Gmail server returned unexpected redirect")]
    UnexpectedRedirect,
}

impl GmailSendError {
    pub fn status(&self) -> Option<u16> {
        match self {
            Self::ApiError { status, .. } => Some(*status),
            _ => None,
        }
    }

    pub fn is_retryable(&self) -> bool {
        matches!(self.status(), Some(429 | 500 | 502 | 503 | 504))
    }
}

#[derive(Debug)]
pub enum GmailSendResult<T> {
    Ok { response: T, keep_alive: bool },
    Io { input: SocketInput },
    Err { err: GmailSendError },
}

pub struct GmailSend<T> {
    send: Http11Send,
    _phantom: PhantomData<T>,
}

impl<T: DeserializeOwned> GmailSend<T> {
    pub fn get(http_auth: &SecretString, url: Url) -> Self {
        Self::with_method(http_auth, "GET", url, None, Vec::new())
    }

    pub fn post_json<B: Serialize>(
        http_auth: &SecretString,
        url: Url,
        body: &B,
    ) -> Result<Self, GmailSendError> {
        let body = serde_json::to_vec(body).map_err(GmailSendError::SerializeRequest)?;
        Ok(Self::with_method(
            http_auth,
            "POST",
            url,
            Some("application/json"),
            body,
        ))
    }

    pub fn put_json<B: Serialize>(
        http_auth: &SecretString,
        url: Url,
        body: &B,
    ) -> Result<Self, GmailSendError> {
        let body = serde_json::to_vec(body).map_err(GmailSendError::SerializeRequest)?;
        Ok(Self::with_method(
            http_auth,
            "PUT",
            url,
            Some("application/json"),
            body,
        ))
    }

    pub fn with_method(
        http_auth: &SecretString,
        method: &str,
        url: Url,
        content_type: Option<&str>,
        body: Vec<u8>,
    ) -> Self {
        let host = url.host_str().unwrap_or("localhost");
        let auth = format!("Bearer {}", http_auth.expose_secret());

        let mut request = HttpRequest::get(url.clone())
            .header("Host", host)
            .header("Accept", "application/json")
            .header("Authorization", auth)
            .body(body);

        if let Some(content_type) = content_type {
            request = request.header("Content-Type", content_type);
        }

        request.method = method.into();

        info!("send Gmail request to {url}");

        Self {
            send: Http11Send::new(request),
            _phantom: PhantomData,
        }
    }

    pub fn delete(http_auth: &SecretString, url: Url) -> Self {
        Self::with_method(http_auth, "DELETE", url, None, Vec::new())
    }

    pub fn resume(&mut self, arg: Option<SocketOutput>) -> GmailSendResult<T> {
        match self.send.resume(arg) {
            Http11SendResult::Ok {
                response,
                keep_alive,
                ..
            } => {
                if response.status.is_success() {
                    let body = if response.body.is_empty() {
                        b"null".as_slice()
                    } else {
                        response.body.as_slice()
                    };

                    match serde_json::from_slice::<T>(body) {
                        Ok(response) => GmailSendResult::Ok {
                            response,
                            keep_alive,
                        },
                        Err(err) => GmailSendResult::Err {
                            err: GmailSendError::ParseResponse(err),
                        },
                    }
                } else {
                    let (status, message) = parse_api_error(*response.status, &response.body);
                    GmailSendResult::Err {
                        err: GmailSendError::ApiError { status, message },
                    }
                }
            }
            Http11SendResult::Io { input } => GmailSendResult::Io { input },
            Http11SendResult::Redirect { .. } => GmailSendResult::Err {
                err: GmailSendError::UnexpectedRedirect,
            },
            Http11SendResult::Err { err } => GmailSendResult::Err { err: err.into() },
        }
    }
}
