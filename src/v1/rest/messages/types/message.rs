//! Gmail message (users.messages).
//!
//! The message resource, together with the base64url codec of its raw
//! RFC 5322 representation.

use alloc::{string::String, vec::Vec};

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use serde::{Deserialize, Serialize};

use crate::v1::rest::messages::GmailMessagePayload;

/// A Gmail message resource.
///
/// Populated fields depend on the requested
/// [`GmailMessageFormat`](crate::v1::rest::messages::GmailMessageFormat):
/// `payload` comes with the full format, `raw` with the raw format.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GmailMessage {
    /// The immutable id of the message.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    /// The id of the thread the message belongs to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    /// The ids of the labels applied to the message.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub label_ids: Vec<String>,
    /// The internal message creation timestamp (epoch milliseconds),
    /// which determines ordering in the inbox.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub internal_date: Option<String>,
    /// A short part of the message text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snippet: Option<String>,
    /// The parsed email structure in the message parts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<GmailMessagePayload>,
    /// The entire message as a base64url-encoded RFC 5322 string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw: Option<String>,
    /// The estimated size of the message in bytes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_estimate: Option<u64>,
    /// The id of the last history record that modified the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub history_id: Option<String>,
}

/// Decodes a base64url-encoded (URL-safe, no padding) raw string back
/// into RFC 5322 message bytes.
///
/// Whitespace and trailing padding are stripped beforehand, so both
/// padded and unpadded inputs decode.
pub fn decode_raw(raw: &str) -> Result<Vec<u8>, base64::DecodeError> {
    let normalized: String = raw.chars().filter(|c| !c.is_ascii_whitespace()).collect();
    let normalized = normalized.trim_end_matches('=');
    URL_SAFE_NO_PAD.decode(normalized)
}

/// Encodes raw RFC 5322 message bytes into the base64url (URL-safe,
/// no padding) string expected by the `raw` field.
pub fn encode_raw(raw: &[u8]) -> String {
    URL_SAFE_NO_PAD.encode(raw)
}
