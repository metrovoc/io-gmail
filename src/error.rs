use alloc::string::{String, ToString};

use serde::Deserialize;
use thiserror::Error;

use crate::send::GmailSendError;

#[derive(Debug, Deserialize)]
struct ErrorEnvelope {
    error: ErrorBody,
}

#[derive(Debug, Deserialize)]
struct ErrorBody {
    code: Option<u16>,
    message: Option<String>,
}

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

#[derive(Debug, Error)]
pub enum GmailError {
    #[error(transparent)]
    Send(#[from] GmailSendError),
    #[error("Decode Gmail RAW message error: {0:?}")]
    DecodeRaw(base64::DecodeError),
}

impl From<base64::DecodeError> for GmailError {
    fn from(err: base64::DecodeError) -> Self {
        Self::DecodeRaw(err)
    }
}
